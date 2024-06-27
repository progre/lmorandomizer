use std::fmt::Write;

use anyhow::{anyhow, Result};
use scraper::{node::Attributes, ElementRef, Html};

use crate::script::data::{
    items::SubWeapon,
    object::{Object, Start},
    script::{Field, Map, World},
    shop_items_data,
};

pub fn parse_script_txt(text: &str) -> Result<(Vec<String>, Vec<World>)> {
    let parser = Html::parse_fragment(text);
    let root = parser.root_element().child_elements().collect::<Vec<_>>();
    // NOTE: scraper converts all tag names to lowercase
    let talks: Vec<_> = root
        .iter()
        .filter(|x| x.value().name() == "talk")
        .map(|x| {
            x.text()
                .collect::<String>()
                .trim_start_matches('\n')
                .to_owned()
        })
        .collect();
    if cfg!(debug_assertions) {
        let first_shop = shop_items_data::parse(&talks[252])?;
        debug_assert_eq!(first_shop.0.number(), SubWeapon::HandScanner as u8);
        debug_assert_eq!(first_shop.0.price(), 20);
        debug_assert_eq!(first_shop.0.set_flag(), 65279);
        debug_assert_eq!(first_shop.1.number(), SubWeapon::Ammunition as u8);
        debug_assert_eq!(first_shop.1.price(), 500);
        debug_assert_eq!(first_shop.1.set_flag(), 65279);
        debug_assert_eq!(first_shop.2.number(), SubWeapon::Buckler as u8);
        debug_assert_eq!(first_shop.2.price(), 80);
        debug_assert_eq!(first_shop.2.set_flag(), 697);
    }
    let worlds: Vec<World> = root
        .iter()
        .filter(|world| world.value().name() == "world")
        .map(|world| {
            Ok(World {
                number: u8::try_from(parse_attrs(&world.value().attrs)?[0])?,
                fields: world
                    .child_elements()
                    .filter(|field| field.value().name() == "field")
                    .map(|field| {
                        let children = flat_children(field);
                        let mut attrs = parse_attrs(&field.value().attrs)?.into_iter();
                        Ok(Field {
                            attrs: (
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                            ),
                            chip_line: {
                                let chip_line = children
                                    .iter()
                                    .find(|child| child.value().name() == "chipline")
                                    .ok_or_else(|| anyhow!("No CHIPLINE found"))?;
                                let attrs = parse_attrs(&chip_line.value().attrs)?;
                                (u16::try_from(attrs[0])?, u16::try_from(attrs[1])?)
                            },
                            hits: children
                                .iter()
                                .filter(|child| child.value().name() == "hit")
                                .map(|&child| {
                                    let attrs = parse_attrs(&child.value().attrs)?;
                                    Ok((i16::try_from(attrs[0])?, i16::try_from(attrs[1])?))
                                })
                                .collect::<Result<_>>()?,
                            animes: children
                                .iter()
                                .filter(|child| child.value().name() == "anime")
                                .map(|&child| {
                                    let attrs = parse_attrs(&child.value().attrs)?;
                                    attrs
                                        .into_iter()
                                        .map(|x| Ok(u16::try_from(x)?))
                                        .collect::<Result<Vec<_>>>()
                                })
                                .collect::<Result<_>>()?,
                            objects: children
                                .iter()
                                .filter(|child| child.value().name() == "object")
                                .map(|&child| parse_object(child))
                                .collect::<Result<_>>()?,
                            maps: children
                                .iter()
                                .filter(|child| child.value().name() == "map")
                                .map(|&child| {
                                    let attrs = parse_attrs(&child.value().attrs)?;
                                    let map_children = flat_children(child);
                                    Ok(Map {
                                        attrs: (
                                            u8::try_from(attrs[0])?,
                                            u8::try_from(attrs[1])?,
                                            u8::try_from(attrs[2])?,
                                        ),
                                        up: {
                                            let up = map_children
                                                .iter()
                                                .find(|x| x.value().name() == "up")
                                                .ok_or_else(|| anyhow!("No UP found"))?;
                                            let attrs = parse_attrs(&up.value().attrs)?;
                                            (
                                                i8::try_from(attrs[0])?,
                                                i8::try_from(attrs[1])?,
                                                i8::try_from(attrs[2])?,
                                                i8::try_from(attrs[3])?,
                                            )
                                        },
                                        right: {
                                            let up = map_children
                                                .iter()
                                                .find(|x| x.value().name() == "right")
                                                .ok_or_else(|| anyhow!("No RIGHT found"))?;
                                            let attrs = parse_attrs(&up.value().attrs)?;
                                            (
                                                i8::try_from(attrs[0])?,
                                                i8::try_from(attrs[1])?,
                                                i8::try_from(attrs[2])?,
                                                i8::try_from(attrs[3])?,
                                            )
                                        },
                                        down: {
                                            let down = map_children
                                                .iter()
                                                .find(|x| x.value().name() == "down")
                                                .ok_or_else(|| anyhow!("No DOWN found"))?;
                                            let attrs = parse_attrs(&down.value().attrs)?;
                                            (
                                                i8::try_from(attrs[0])?,
                                                i8::try_from(attrs[1])?,
                                                i8::try_from(attrs[2])?,
                                                i8::try_from(attrs[3])?,
                                            )
                                        },
                                        left: {
                                            let left = map_children
                                                .iter()
                                                .find(|x| x.value().name() == "left")
                                                .ok_or_else(|| anyhow!("No LEFT found"))?;
                                            let attrs = parse_attrs(&left.value().attrs)?;
                                            (
                                                i8::try_from(attrs[0])?,
                                                i8::try_from(attrs[1])?,
                                                i8::try_from(attrs[2])?,
                                                i8::try_from(attrs[3])?,
                                            )
                                        },
                                        objects: map_children
                                            .iter()
                                            .filter(|object| object.value().name() == "object")
                                            .map(|&object| parse_object(object))
                                            .collect::<Result<_>>()?,
                                    })
                                })
                                .collect::<Result<_>>()?,
                        })
                    })
                    .collect::<Result<_>>()?,
            })
        })
        .collect::<Result<_>>()?;

    debug_assert_eq!(talks.len(), 905);
    debug_assert_eq!(worlds[0].fields[0].objects[0].starts[0].flag, 99999);
    debug_assert_eq!(worlds[0].fields[0].maps[0].objects[5].starts[0].flag, 58);
    Ok((talks, worlds))
}

fn parse_attrs(attrs: &Attributes) -> Result<Vec<i32>> {
    Ok(attrs
        .keys()
        .next()
        .ok_or(anyhow!("No attributes found"))?
        .local
        .split(',')
        .map(|x| x.parse::<i32>())
        .collect::<Result<_, _>>()?)
}

fn flat_children(root: ElementRef) -> Vec<ElementRef> {
    root.child_elements()
        .map(|x| {
            if x.child_elements().count() == 0 {
                return vec![x];
            }
            if x.value().name() == "object" || x.value().name() == "map" {
                return vec![x];
            }
            let mut vec = vec![x];
            vec.append(&mut flat_children(x));
            vec
        })
        .reduce(|mut p, mut c| {
            p.append(&mut c);
            p
        })
        .unwrap_or_default()
}

fn parse_object(object: ElementRef) -> Result<Object> {
    let attrs = parse_attrs(&object.value().attrs)?;
    Ok(Object {
        number: u16::try_from(attrs[0])?,
        x: attrs[1],
        y: attrs[2],
        op1: attrs[3],
        op2: attrs[4],
        op3: attrs[5],
        op4: attrs[6],
        starts: flat_children(object)
            .iter()
            .map(|x| {
                let start_attrs = parse_attrs(&x.value().attrs)?;
                Ok(Start {
                    flag: u32::try_from(start_attrs[0])?,
                    run_when_unset: start_attrs[1] != 0,
                })
            })
            .collect::<Result<_>>()?,
    })
}

pub fn stringify_script_txt(talks: &[String], worlds: &[World]) -> String {
    [
        talks.iter().fold(String::new(), |mut output, x| {
            write!(output, "<TALK>\n{x}</TALK>\n").unwrap();
            output
        }),
        worlds
            .iter()
            .map(|world| {
                [
                    format!("<WORLD {}>\n", world.number),
                    world
                        .fields
                        .iter()
                        .map(|field| {
                            [
                                format!(
                                    "<FIELD {},{},{},{},{}>\n",
                                    field.attrs.0,
                                    field.attrs.1,
                                    field.attrs.2,
                                    field.attrs.3,
                                    field.attrs.4
                                ),
                                format!("<CHIPLINE {},{}>\n", field.chip_line.0, field.chip_line.1),
                                field.hits.iter().fold(String::new(), |mut output, (x, y)| {
                                    writeln!(output, "<HIT {},{}>", x, y).unwrap();
                                    output
                                }),
                                field
                                    .animes
                                    .iter()
                                    .fold(String::new(), |mut output, anime| {
                                        writeln!(
                                            output,
                                            "<ANIME {}>",
                                            anime
                                                .iter()
                                                .map(|x| x.to_string())
                                                .collect::<Vec<_>>()
                                                .join(",")
                                        )
                                        .unwrap();
                                        output
                                    }),
                                field
                                    .objects
                                    .iter()
                                    .map(stringify_object)
                                    .collect::<String>(),
                                field
                                    .maps
                                    .iter()
                                    .map(|map| {
                                        [
                                            format!(
                                                "<MAP {},{},{}>\n",
                                                map.attrs.0, map.attrs.1, map.attrs.2,
                                            ),
                                            format!(
                                                "<UP {},{},{},{}>\n",
                                                map.up.0, map.up.1, map.up.2, map.up.3,
                                            ),
                                            format!(
                                                "<RIGHT {},{},{},{}>\n",
                                                map.right.0, map.right.1, map.right.2, map.right.3,
                                            ),
                                            format!(
                                                "<DOWN {},{},{},{}>\n",
                                                map.down.0, map.down.1, map.down.2, map.down.3,
                                            ),
                                            format!(
                                                "<LEFT {},{},{},{}>\n",
                                                map.left.0, map.left.1, map.left.2, map.left.3,
                                            ),
                                            map.objects
                                                .iter()
                                                .map(stringify_object)
                                                .collect::<String>(),
                                            "</MAP>\n".to_owned(),
                                        ]
                                        .join("")
                                    })
                                    .collect::<String>(),
                                "</FIELD>\n".to_owned(),
                            ]
                            .join("")
                        })
                        .collect::<String>(),
                    "</WORLD>\n".to_owned(),
                ]
                .join("")
            })
            .collect::<String>(),
    ]
    .join("")
}

fn stringify_object(object: &Object) -> String {
    format!(
        "<OBJECT {},{},{},{},{},{},{}>\n{}</OBJECT>\n",
        object.number,
        object.x,
        object.y,
        object.op1,
        object.op2,
        object.op3,
        object.op4,
        object
            .starts
            .iter()
            .fold(String::new(), |mut output, start| {
                writeln!(
                    output,
                    "<START {},{}>",
                    start.flag,
                    if start.run_when_unset { 1 } else { 0 }
                )
                .unwrap();
                output
            }),
    )
}
