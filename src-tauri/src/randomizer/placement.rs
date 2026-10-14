use super::storage::{
    ids::{ItemId, SpotId},
    item::StrategyFlag,
};

/// 1 回の探索で到達可能になった spot に置いた item と、達成した event
pub struct Sphere<'a> {
    pub items: Vec<(SpotId, ItemId)>,
    pub events: Vec<&'a StrategyFlag>,
}

/// 探索で決まった配置。書き換え後の `Storage` と spoiler log はここから作る
pub struct Placement<'a> {
    pub progression: Vec<Sphere<'a>>,
    pub maps: Vec<(SpotId, ItemId)>,
}

impl Placement<'_> {
    pub fn all_items(&self) -> impl Iterator<Item = &(SpotId, ItemId)> {
        self.progression
            .iter()
            .flat_map(|sphere| &sphere.items)
            .chain(&self.maps)
    }

    #[cfg(test)]
    pub fn count_checkpoints(&self) -> usize {
        self.progression
            .iter()
            .map(|sphere| sphere.items.len() + sphere.events.len())
            .sum::<usize>()
            + self.maps.len()
    }
}
