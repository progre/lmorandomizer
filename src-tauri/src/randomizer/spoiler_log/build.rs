use crate::{
    dataset::regions::Regions,
    randomizer::{
        placement::Placement,
        spoiler::spots::{SpotRef, Spots},
        storage::{
            Storage,
            ids::{ItemId, SpotId, SpotTable},
            item::Item,
        },
    },
    script::enums::{ChestItem, FieldNumber, ShopItem},
};

use super::{Line, Sphere, SpoilerLog};

fn compare_key_for_spoiler_log(field_number: FieldNumber) -> u8 {
    if matches!(field_number, FieldNumber::TwinLabyrinthsRight) {
        FieldNumber::TwinLabyrinthsLeft.to_logic_number().unwrap() * 10 + 1
    } else {
        field_number.to_logic_number().unwrap() * 10
    }
}

/// field 順、spot の種類順、種類ごとの番号順に並べるための key
fn sort_key(spot: SpotRef, regions: &Regions, shop_list: &[[Option<ShopItem>; 3]]) -> usize {
    let (type_num, src_idx) = match spot {
        SpotRef::MainWeapon(x) => (1, x.spot.main_weapon() as usize),
        SpotRef::SubWeapon(x) => (2, x.spot.sub_weapon() as usize),
        SpotRef::Chest(x) => match x.spot.item() {
            ChestItem::Equipment(equipment) => (3, equipment as usize),
            ChestItem::Rom(rom) => (3, 100 + rom as usize),
        },
        SpotRef::Seal(x) => (4, x.spot.seal() as usize),
        SpotRef::Rom(_) => (5, 0),
        SpotRef::Talk(_) => (6, 0),
        SpotRef::Shop(x) => (
            7,
            shop_list.iter().position(|&y| y == x.spot.items()).unwrap(),
        ),
    };
    let field = regions.get(spot.region()).field_number();
    compare_key_for_spoiler_log(field) as usize * 10000 + type_num * 1000 + src_idx
}

fn line(spot: SpotRef, item: &Item, regions: &Regions) -> Line {
    let item = item.name.get().to_owned();
    let display = spot.display(regions).to_string();
    match spot {
        SpotRef::MainWeapon(_)
        | SpotRef::SubWeapon(_)
        | SpotRef::Chest(_)
        | SpotRef::Seal(_)
        | SpotRef::Rom(_)
        | SpotRef::Talk(_) => Line::Spot {
            spot: display,
            item,
        },
        SpotRef::Shop(x) => Line::Shop {
            shop: display,
            idx: x.idx,
            item,
        },
    }
}

struct Builder<'a> {
    spots: SpotTable<SpotRef<'a>>,
    regions: &'a Regions,
}

impl<'a> Builder<'a> {
    fn placed(&self, &(spot, item): &(SpotId, ItemId)) -> (SpotRef<'a>, &'a Item) {
        let spot = *self.spots.get(spot).unwrap();
        let item = self.spots.get(item.origin()).unwrap().item();
        (spot, item)
    }

    fn sorted_lines(&self, items: &[(SpotId, ItemId)]) -> Vec<Line> {
        let mut placed: Vec<_> = items.iter().map(|x| self.placed(x)).collect();
        let shop_list: Vec<_> = placed
            .iter()
            .filter_map(|(spot, _)| match spot {
                SpotRef::Shop(x) => Some(x.spot.items()),
                SpotRef::MainWeapon(_)
                | SpotRef::SubWeapon(_)
                | SpotRef::Chest(_)
                | SpotRef::Seal(_)
                | SpotRef::Rom(_)
                | SpotRef::Talk(_) => None,
            })
            .collect();
        placed.sort_by_key(|(spot, _)| sort_key(*spot, self.regions, &shop_list));
        placed
            .into_iter()
            .map(|(spot, item)| line(spot, item, self.regions))
            .collect()
    }
}

impl SpoilerLog {
    pub fn new(placement: &Placement, source: &Storage) -> Self {
        let builder = Builder {
            spots: Spots::new(source).to_table(),
            regions: &source.regions,
        };
        let progression = placement
            .progression
            .iter()
            .map(|sphere| Sphere {
                lines: builder.sorted_lines(&sphere.items),
                events: sphere.events.iter().map(|x| x.get().to_owned()).collect(),
            })
            .collect();
        let maps = placement
            .maps
            .iter()
            .map(|x| builder.placed(x))
            .map(|(spot, item)| line(spot, item, builder.regions))
            .collect();
        Self { progression, maps }
    }
}
