use std::thread::spawn;

use crate::{
    dataset::storage::{ItemSpot, Storage},
    randomizer::spoiler_log::{Checkpoint, Sphere},
};

use super::spoiler_log::SpoilerLog;

pub fn split_reachables_unreachables(
    storage: Storage,
    current_checkpoints: &[Checkpoint],
    current_sacred_orb_count: u8,
) -> (Vec<Checkpoint>, Storage) {
    let (reached_checkpoints_tx, reached_item_names_rx) = std::sync::mpsc::channel();

    let (main_weapon_shutters, sub_weapon_shutters, chests, seal_chests, shops) =
        storage.into_inner();

    let f = |item_spots: Vec<ItemSpot>| {
        let current_checkpoints = current_checkpoints.to_owned();
        let reached_checkpoints_tx = reached_checkpoints_tx.clone();
        move || {
            let mut unreached_item_spots = Vec::new();
            for item_spot in item_spots {
                if !item_spot.spot.is_reachable(
                    current_checkpoints.iter().flat_map(|x| &x.strategy_flag),
                    current_sacred_orb_count,
                ) {
                    unreached_item_spots.push(item_spot);
                    continue;
                }
                let checkpoint = Checkpoint {
                    spot: item_spot.spot,
                    strategy_flag: vec![item_spot.item.name().to_owned()],
                };
                reached_checkpoints_tx.send(checkpoint).unwrap();
            }
            unreached_item_spots
        }
    };
    let main_weapon_shutter_handle = spawn(f(main_weapon_shutters));
    let sub_weapon_shutters_handle = spawn(f(sub_weapon_shutters));
    let chests_handle = spawn(f(chests));
    let seal_chests_handle = spawn(f(seal_chests));
    let shops_handle = spawn({
        let current_checkpoints = current_checkpoints.to_owned();
        let reached_checkpoints_tx = reached_checkpoints_tx.clone();
        move || {
            let mut unreached_shops = Vec::new();
            for shop in shops {
                if !shop.spot.is_reachable(
                    current_checkpoints.iter().flat_map(|x| &x.strategy_flag),
                    current_sacred_orb_count,
                ) {
                    unreached_shops.push(shop);
                    continue;
                }
                let checkpoint = Checkpoint {
                    spot: shop.spot.clone(),
                    strategy_flag: vec![
                        shop.items.0.name().to_owned(),
                        shop.items.1.name().to_owned(),
                        shop.items.2.name().to_owned(),
                    ],
                };
                reached_checkpoints_tx.send(checkpoint).unwrap();
            }
            unreached_shops
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

    (reached_item_names_rx.iter().collect(), storage)
}

pub fn validate(storage: &Storage) -> Option<SpoilerLog> {
    let mut current_item_names: Vec<Checkpoint> = Default::default();
    let mut playing = storage.clone();
    let mut progression = Vec::new();
    for _ in 0..100 {
        let current_sacred_orb_count = current_item_names
            .iter()
            .flat_map(|x| &x.strategy_flag)
            .filter(|x| x.is_sacred_orb())
            .count() as u8;
        let result =
            split_reachables_unreachables(playing, &current_item_names, current_sacred_orb_count);
        let mut reached = result.0;
        if reached.is_empty() {
            return None;
        }
        playing = result.1;

        if playing.all_items().is_empty() {
            return Some(SpoilerLog { progression });
        }
        progression.push(Sphere(reached.clone()));
        current_item_names.append(&mut reached);
    }
    unreachable!();
}
