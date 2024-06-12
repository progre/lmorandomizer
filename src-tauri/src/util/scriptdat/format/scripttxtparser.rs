use std::fmt::Write;

use anyhow::{anyhow, Result};
use scraper::{node::Attributes, ElementRef, Html};

use crate::{
    randomizer::items::SubWeaponNumber,
    util::scriptdat::{
        data::{
            lm_object::{LMObject, LMStart},
            script::{LMChild, LMField, LMMap, LMWorld},
        },
        format::shop_items_data,
    },
};

pub fn parse_script_txt(text: &str) -> Result<(Vec<String>, Vec<LMWorld>)> {
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
        let first_shop = shop_items_data::parse(&talks[252]);
        debug_assert_eq!(first_shop.0.number, SubWeaponNumber::HandScanner as i8);
        debug_assert_eq!(first_shop.0.price, 20);
        debug_assert_eq!(first_shop.0.flag, 65279);
        debug_assert_eq!(first_shop.1.number, SubWeaponNumber::Ammunition as i8);
        debug_assert_eq!(first_shop.1.price, 500);
        debug_assert_eq!(first_shop.1.flag, 65279);
        debug_assert_eq!(first_shop.2.number, SubWeaponNumber::Buckler as i8);
        debug_assert_eq!(first_shop.2.price, 80);
        debug_assert_eq!(first_shop.2.flag, 697);
    }
    let worlds: Vec<LMWorld> = root
        .iter()
        .filter(|world| world.value().name() == "world")
        .map(|world| {
            Ok(LMWorld {
                value: u8::try_from(parse_attrs(&world.value().attrs)?[0])?,
                fields: world
                    .child_elements()
                    .filter(|field| field.value().name() == "field")
                    .map(|field| {
                        let children = flat_children(field);
                        let mut attrs = parse_attrs(&field.value().attrs)?.into_iter();
                        Ok(LMField {
                            attrs: (
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                                u8::try_from(attrs.next().ok_or(anyhow!("No attributes found"))?)?,
                            ),
                            children: children
                                .iter()
                                .filter(|child| {
                                    child.value().name() != "object"
                                        && child.value().name() != "map"
                                })
                                .map(|&child| parse_child(child))
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
                                    Ok(LMMap {
                                        attrs: (
                                            u8::try_from(attrs[0])?,
                                            u8::try_from(attrs[1])?,
                                            u8::try_from(attrs[2])?,
                                        ),
                                        children: map_children
                                            .iter()
                                            .filter(|x| x.value().name() != "object")
                                            .map(|&child| parse_child(child))
                                            .collect::<Result<_>>()?,
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
    debug_assert_eq!(worlds[0].fields[0].objects[0].starts[0].number, 99999);
    debug_assert_eq!(worlds[0].fields[0].maps[0].objects[5].starts[0].number, 58);
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

fn parse_child(child: ElementRef) -> Result<LMChild> {
    Ok(LMChild {
        name: child.value().name().to_owned(),
        attrs: parse_attrs(&child.value().attrs)?,
    })
}

fn parse_object(object: ElementRef) -> Result<LMObject> {
    let attrs = parse_attrs(&object.value().attrs)?;
    Ok(LMObject {
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
                Ok(LMStart {
                    number: start_attrs[0],
                    value: start_attrs[1] != 0,
                })
            })
            .collect::<Result<_>>()?,
    })
}

pub fn stringify_script_txt(talks: &[String], worlds: &[LMWorld]) -> String {
    [
        talks.iter().fold(String::new(), |mut output, x| {
            write!(output, "<TALK>\n{x}</TALK>\n").unwrap();
            output
        }),
        worlds
            .iter()
            .map(|world| {
                [
                    format!("<WORLD {}>\n", world.value),
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
                                field
                                    .children
                                    .iter()
                                    .map(stringify_child)
                                    .collect::<String>(),
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
                                            map.children
                                                .iter()
                                                .map(stringify_child)
                                                .collect::<String>(),
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

fn stringify_child(child: &LMChild) -> String {
    format!(
        "<{} {}>\n",
        child.name.to_uppercase(),
        child
            .attrs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn stringify_object(object: &LMObject) -> String {
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
                    start.number,
                    if start.value { 1 } else { 0 }
                )
                .unwrap();
                output
            }),
    )
}
