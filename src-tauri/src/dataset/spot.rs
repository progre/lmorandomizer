use super::item::Item;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Spot {
    /// 'weaponShutter' | 'chest' | 'shop' | 'sealChest'
    pub r#type: String,
    pub requirement_items: Option<Vec<Vec<Item>>>,
    pub talk_number: Option<u16>,
}

impl Spot {
    pub fn new(
        r#type: String,
        requirement_items: Option<Vec<Vec<Item>>>,
        talk_number: Option<u16>,
    ) -> Self {
        if cfg!(debug_assertions) {
            if r#type == "shop" {
                debug_assert_ne!(talk_number, None);
            } else {
                debug_assert_eq!(talk_number, None);
            }
        }
        Self {
            r#type,
            requirement_items,
            talk_number,
        }
    }

    pub fn is_reachable(&self, current_item_names: &[String], sacred_orb_count: usize) -> bool {
        let Some(requirement_items) = &self.requirement_items else {
            return true;
        };
        requirement_items.iter().any(|group| {
            group.iter().all(|x| {
                x.name == "sacredOrb" && x.count as usize <= sacred_orb_count
                    || current_item_names.contains(&&x.name)
            })
        })
    }
}
