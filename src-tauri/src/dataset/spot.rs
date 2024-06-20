use std::collections::HashSet;

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

    pub fn is_reachable(
        &self,
        current_item_names: &HashSet<StrategyFlag>,
        sacred_orb_count: u8,
    ) -> bool {
        let Some(any) = &self.requirement_items else {
            return true;
        };
        any.0.iter().any(|all| {
            all.0.iter().all(|x| {
                x.is_sacred_orb() && x.sacred_orb_count() <= sacred_orb_count
                    || current_item_names.contains(x)
            })
        })
    }
}
