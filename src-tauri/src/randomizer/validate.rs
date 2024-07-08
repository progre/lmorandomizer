use std::{collections::HashSet, thread::spawn};

use crate::{
    dataset::{
        storage::{ItemSpot, Storage},
        supplements::StrategyFlag,
    },
    randomizer::spoiler_log::{Checkpoint, Sphere},
};

use super::spoiler_log::SpoilerLog;

fn partition_reachables_unreachables(
    storage: Storage,
    current_strategy_flags: &[StrategyFlag],
    current_sacred_orb_count: u8,
) -> (Vec<Checkpoint>, Storage) {
    let (reached_checkpoints_tx, reached_item_names_rx) = std::sync::mpsc::channel();

    let (main_weapon_shutters, sub_weapon_shutters, chests, seal_chests, shops) =
        storage.into_inner();

    let f = |item_spots: Vec<ItemSpot>| {
        let current_strategy_flags = current_strategy_flags.to_owned();
        let reached_checkpoints_tx = reached_checkpoints_tx.clone();
        move || {
            let current_strategy_flags: HashSet<_> =
                current_strategy_flags.iter().map(|x| x.get()).collect();

            let (reachables, unreachables) =
                item_spots.into_iter().partition::<Vec<_>, _>(|item_spot| {
                    item_spot
                        .spot
                        .is_reachable(&current_strategy_flags, current_sacred_orb_count)
                });
            let reached_checkpoints: Vec<_> = reachables
                .into_iter()
                .map(|item_spot| Checkpoint {
                    spot: item_spot.spot.clone(),
                    strategy_flags: vec![item_spot.item.name().to_owned()],
                })
                .collect();
            reached_checkpoints_tx.send(reached_checkpoints).unwrap();
            unreachables
        }
    };
    let main_weapon_shutter_handle = spawn(f(main_weapon_shutters));
    let sub_weapon_shutters_handle = spawn(f(sub_weapon_shutters));
    let chests_handle = spawn(f(chests));
    let seal_chests_handle = spawn(f(seal_chests));
    let shops_handle = spawn({
        let current_strategy_flags = current_strategy_flags.to_owned();
        let reached_checkpoints_tx = reached_checkpoints_tx.clone();
        move || {
            let current_strategy_flags: HashSet<_> =
                current_strategy_flags.iter().map(|x| x.get()).collect();

            let (reachables, unreachables) = shops.into_iter().partition::<Vec<_>, _>(|shop| {
                shop.spot
                    .is_reachable(&current_strategy_flags, current_sacred_orb_count)
            });
            let reached_checkpoints: Vec<_> = reachables
                .into_iter()
                .map(|shop| Checkpoint {
                    spot: shop.spot.clone(),
                    strategy_flags: vec![
                        shop.items.0.name().to_owned(),
                        shop.items.1.name().to_owned(),
                        shop.items.2.name().to_owned(),
                    ],
                })
                .collect();
            reached_checkpoints_tx.send(reached_checkpoints).unwrap();
            unreachables
        }
    });

    let storage = Storage::new(
        main_weapon_shutter_handle.join().unwrap(),
        sub_weapon_shutters_handle.join().unwrap(),
        chests_handle.join().unwrap(),
        seal_chests_handle.join().unwrap(),
        shops_handle.join().unwrap(),
    );

    drop(reached_checkpoints_tx);

    (reached_item_names_rx.iter().flatten().collect(), storage)
}

pub fn validate(storage: &Storage) -> Option<SpoilerLog> {
    let mut current_strategy_flags: Vec<StrategyFlag> = Default::default();
    let mut playing = storage.clone();
    let mut progression = Vec::new();
    for _ in 0..100 {
        let current_sacred_orb_count = current_strategy_flags
            .iter()
            .filter(|x| x.is_sacred_orb())
            .count() as u8;
        let (reached, unreached) = partition_reachables_unreachables(
            playing,
            &current_strategy_flags,
            current_sacred_orb_count,
        );
        if reached.is_empty() {
            return None;
        }
        playing = unreached;

        if playing.all_items().count() == 0 {
            return Some(SpoilerLog { progression });
        }
        progression.push(Sphere(reached.clone()));
        reached
            .into_iter()
            .flat_map(|x| x.strategy_flags)
            .for_each(|x| current_strategy_flags.push(x));
    }
    unreachable!();
}
