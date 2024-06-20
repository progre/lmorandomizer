use std::collections::HashSet;

use crate::dataset::{storage::Storage, supplements::StrategyFlag};

pub fn validate(storage: &Storage) -> bool {
    let mut current_item_names: HashSet<StrategyFlag> = Default::default();
    let mut playing = storage.clone();
    for _ in 0..100 {
        let current_sacred_orb_count = current_item_names
            .iter()
            .filter(|x| x.is_sacred_orb())
            .count() as u8;
        let result =
            playing.split_reachables_unreachables(&current_item_names, current_sacred_orb_count);
        let reached = result.0;
        if reached.is_empty() {
            return false;
        }
        playing = result.1;

        if playing.all_items().is_empty() {
            log::trace!("true");
            return true;
        }
        for item in reached {
            current_item_names.insert(item);
        }
    }
    unreachable!();
}
