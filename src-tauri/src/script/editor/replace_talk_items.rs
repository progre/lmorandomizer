use std::{collections::HashMap, ops::Deref};

use anyhow::{anyhow, bail, Result};
use log::warn;

use crate::{
    randomizer::storage,
    script::{
        data::{
            item::Item,
            object::Shop,
            script::Script,
            talk::{read_u16, write_u16, Talk},
        },
        editor::script_editor::find_item_set_flag,
        enums::{Equipment, Rom, TalkItem},
    },
};

fn to_name_talk_number(item: TalkItem) -> usize {
    match item {
        TalkItem::Rom(rom) => rom as usize,
        TalkItem::Equipment(equipment) => 500 + equipment as usize,
    }
}

pub fn to_name(item: TalkItem, talks: &[Talk]) -> Result<Talk> {
    let talk_number = to_name_talk_number(item);
    let Some(talk) = talks.get(talk_number).cloned() else {
        bail!("script broken: talk_number={}", talk_number)
    };
    Ok(talk)
}

fn replace_item(
    talks: &mut [Talk],
    talk_number: usize,
    new_item: TalkItem,
    set_flag: u16,
) -> Result<()> {
    let talk = &talks[talk_number];
    let (old_item, _) = talk
        .item()?
        .ok_or_else(|| anyhow!("item not found: {:?}", talk.to_string()))?;
    if old_item == new_item {
        return Ok(());
    }
    let talk = &mut talks[talk_number];
    for range in talk.control_talk_command_ranges() {
        let cmd = &mut talk.as_bytes_mut()[range];
        match cmd[0] {
            2 => (cmd[1], cmd[2]) = write_u16(set_flag),
            4 | 5 => {
                (cmd[0], cmd[1]) = match new_item {
                    TalkItem::Equipment(e) => (4, e as u8 + 1),
                    TalkItem::Rom(r) => (5, r as u8 + 1),
                };
            }
            _ => {}
        }
    }
    let talk = &talks[talk_number];
    let string = talk.to_string();
    let old_item_name = to_name(old_item, talks)?.to_string();
    let new_item_name = to_name(new_item, talks)?;

    const ITEM_NAME_MAP: [(TalkItem, &str); 4] = [
        (TalkItem::Equipment(Equipment::Anchor), "ｶﾞﾗｸﾀ"),
        (TalkItem::Equipment(Equipment::MiniDoll), "ｺﾋﾞﾄの人形"),
        (
            TalkItem::Equipment(Equipment::MulanaTalisman),
            "ﾗﾑﾗｰﾅのこﾞふ",
        ),
        (TalkItem::Rom(Rom::Pr3), "ひとつ"),
    ];
    let old_item_name = ITEM_NAME_MAP
        .iter()
        .filter(|(key, _)| key == &old_item)
        .map(|&(_, value)| value)
        .next()
        .unwrap_or_else(|| &old_item_name);

    let result = string.replace(old_item_name, &new_item_name.to_string());
    if result == string {
        warn!(
            "failed to replace item: talk_number={}, old_item_name={}, new_item_name={}",
            talk_number, old_item_name, new_item_name,
        );
    }
    let talk = &mut talks[talk_number];
    *talk = Talk::from_text(&result);
    Ok(())
}

fn replace_flag_map(storage_talks: &[storage::Talk], script: &Script) -> Result<HashMap<u16, u16>> {
    storage_talks
        .iter()
        .map(|talk| {
            let old_flag = find_item_set_flag(script, talk.spot.item())?
                .ok_or_else(|| anyhow!("talk not found: {:?}", talk.spot.item()))?;
            let new_flag = Item::new(&talk.item.src, script).unwrap().flag();
            Ok((old_flag, new_flag))
        })
        .collect()
}

pub fn replace_talk_items(
    talks: &mut [Talk],
    script: &Script,
    storage_talks: &[storage::Talk],
) -> Result<()> {
    let shops = script
        .shops()
        .map(|x| Shop::try_from_shop_object(x, talks.deref()))
        .collect::<Result<Vec<_>>>()?;

    let storyteller_talk_numbers = shops
        .iter()
        .filter_map(|x| match x {
            Shop::Storyteller(x) => Some(vec![x.talk_number()]),
            Shop::ItemShop(_) => None,
            Shop::Eldest(x) => Some(x.important_talk_numbers().to_vec()),
        })
        .flatten()
        .collect::<Vec<_>>();
    let storage_talk_spot_item_pairs = storage_talks
        .iter()
        .map(|storage_talk| {
            let spot = storage_talk.spot.item();
            let (item, set_flag) = match Item::new(&storage_talk.item.src, script)? {
                Item::Equipment(e) => (TalkItem::Equipment(e.content), e.flag),
                Item::Rom(r) => (TalkItem::Rom(r.content), r.flag),
                Item::MainWeapon(_) | Item::SubWeapon(_) | Item::Seal(_) => unreachable!(),
            };
            Ok((spot, item, set_flag))
        })
        .collect::<Result<Vec<_>>>()?;

    for talk_number in (0..talks.len())
        .filter(|talk_number| storyteller_talk_numbers.contains(&(*talk_number as u16)))
    {
        let talk_item = talks[talk_number].item()?;
        for &(_spot, item, set_flag) in storage_talk_spot_item_pairs
            .iter()
            .filter(|&(spot, _, _)| matches!(talk_item, Some((talk_item, _)) if &talk_item == spot))
        {
            replace_item(talks, talk_number, item, set_flag)?;
        }
    }

    let replace_flag_map = replace_flag_map(storage_talks, script)?;
    shops
        .into_iter()
        .filter_map(|x| match x {
            Shop::Storyteller(_) => None,
            Shop::ItemShop(_) => None,
            Shop::Eldest(x) => Some(x.into_important_talk_cond_talk_numbers()),
        })
        .flatten()
        .map(|talk_number| {
            let cond = talks[talk_number as usize].as_bytes_mut();
            let mut i = 2;
            while i < cond.len() {
                let current = i;
                if cond[current] == 2 {
                    i += 2;
                    continue;
                }
                i += 3;
                if cond[current] != 1 {
                    bail!("invalid talk command: {:x?}", cond);
                }
                let old_off_flag = read_u16(cond[current + 1], cond[current + 2]);
                let Some(&new_off_flag) = replace_flag_map.get(&old_off_flag) else {
                    continue;
                };
                (cond[current + 1], cond[current + 2]) = write_u16(new_off_flag);
            }
            Ok(())
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}
