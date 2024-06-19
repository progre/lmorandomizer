use crate::dataset::storage::Storage;

pub fn validate(storage: &Storage, all_requirement_names: &[String]) -> bool {
    // スポット名 sacredOrb と要求名 sacredOrb:~~~ のバリデーション？
    // TODO: なぜか sacredOrb が初期アイテム扱いになってるが、多分バグ
    let mut current_item_names: Vec<_> = all_requirement_names
        .iter()
        .filter(|&x| storage.all_items().iter().map(|y| &y.name).all(|y| y != x))
        .cloned()
        .collect();
    debug_assert_eq!(current_item_names, vec!["sacredOrb"]);
    current_item_names = vec![];
    let mut playing = storage.clone();
    for _ in 0..100 {
        let sacred_orb_count = current_item_names
            .iter()
            .filter(|x| x.starts_with("sacredOrb:"))
            .count() as u8;
        let result = playing.split_reachables_unreachables(&current_item_names, sacred_orb_count);
        let mut reached = result.0;
        if reached.is_empty() {
            return false;
        }
        playing = result.1;

        if playing.all_items().is_empty() {
            log::trace!("true");
            return true;
        }
        current_item_names.append(&mut reached);
    }
    unreachable!();
}
