use super::item::Item;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Spot {
    requirement_items: Option<Vec<Vec<Item>>>,
}

impl Spot {
    pub fn new(requirement_items: Option<Vec<Vec<Item>>>) -> Self {
        Self { requirement_items }
    }

    pub fn requirement_items(&self) -> Option<&Vec<Vec<Item>>> {
        self.requirement_items.as_ref()
    }

    pub fn is_reachable(&self, current_item_names: &[String], sacred_orb_count: u8) -> bool {
        let Some(requirement_items) = &self.requirement_items else {
            return true;
        };
        requirement_items.iter().any(|group| {
            group.iter().all(|x| {
                x.name == "sacredOrb" && x.count <= sacred_orb_count
                    || current_item_names.contains(&x.name)
            })
        })
    }
}
