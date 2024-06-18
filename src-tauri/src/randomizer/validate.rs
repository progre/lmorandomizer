use crate::dataset::storage::Storage;

pub fn validate(storage: &Storage) -> bool {
    let mut current_item_names: Vec<_> = storage
        .all_requirement_names()
        .iter()
        .filter(|&x| !storage.all_items().iter().map(|y| &y.name).any(|y| y == x))
        .cloned()
        .collect();
    debug_assert_eq!(current_item_names, vec!["sacredOrb"]);
    current_item_names = vec![];
    let mut playing = storage.clone();
    for _ in 0..100 {
        let sacred_orb_count = current_item_names
            .iter()
            .filter(|x| x.starts_with("sacredOrb:"))
            .count();
        let reached = playing.reachable_item_names(&current_item_names, sacred_orb_count);
        if reached.is_empty() {
            return false;
        }
        playing = playing.unreachables(&current_item_names, sacred_orb_count);

        if playing.all_items().is_empty() {
            log::trace!("true");
            return true;
        }
        current_item_names = current_item_names
            .into_iter()
            .chain(reached.into_iter())
            .collect();
    }
    unreachable!();
}
