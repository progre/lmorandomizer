use super::supplements::StrategyFlag;

#[derive(Clone, Debug, PartialEq)]
pub struct AllRequirements(pub Vec<StrategyFlag>);

#[derive(Clone, Debug, PartialEq)]
pub struct AnyOfAllRequirements(pub Vec<AllRequirements>);

#[derive(Clone, Debug)]
pub struct Spot {
    requirement_items: Option<AnyOfAllRequirements>,
}

impl Spot {
    pub fn new(requirement_items: Option<AnyOfAllRequirements>) -> Self {
        Self { requirement_items }
    }

    pub fn is_reachable(&self, current_item_names: &[String], sacred_orb_count: u8) -> bool {
        let Some(requirement_items) = &self.requirement_items else {
            return true;
        };
        requirement_items.0.iter().any(|group| {
            group.0.iter().all(|x| {
                x.is_sacred_orb() && x.sacred_orb_count() <= sacred_orb_count
                    || current_item_names.contains(&x.0)
            })
        })
    }
}
