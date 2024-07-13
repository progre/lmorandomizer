use rand::{seq::SliceRandom, Rng};

use crate::dataset::{item::Item, spot::Spot, storage::Shop};

fn pick_items_include_requires(
    rng: &mut impl Rng,
    items_pool: &mut Vec<Item>,
    spots: &[&Spot],
    count: usize,
) -> Vec<Item> {
    let Some(pos) = items_pool.iter().position(|item| item.is_required(spots)) else {
        return (0..count).map(|_| items_pool.pop().unwrap()).collect();
    };
    let req_item = items_pool.swap_remove(pos);
    let mut field_items = vec![req_item];
    (0..count - 1).for_each(|_| {
        field_items.push(items_pool.pop().unwrap());
    });
    debug_assert!(!field_items.is_empty());
    field_items.as_mut_slice().shuffle(rng);
    field_items
}

fn distribute_priority_items(
    rng: &mut impl Rng,
    priority_items: Vec<Item>,
    mut field_spot_count: usize,
    mut shop_display_count: usize,
) -> (Vec<Item>, Vec<Item>) {
    let mut field_items = Vec::new();
    let mut shop_items = Vec::new();
    for item in priority_items {
        let numerator = field_spot_count as u32;
        let denominator = (field_spot_count + shop_display_count) as u32;
        if rng.gen_ratio(numerator, denominator) {
            field_items.push(item);
            field_spot_count -= 1;
        } else {
            shop_items.push(item);
            shop_display_count -= 1;
        }
    }
    (field_items, shop_items)
}

pub struct ItemsPool {
    pub priority_items: Option<Vec<Item>>,
    pub field_items: Vec<Item>,
    pub shop_items: Vec<Item>,
    pub consumable_items: Vec<Item>,
}

pub fn pickup_items<'a>(
    rng: &mut impl Rng,
    items_pool: &mut ItemsPool,
    (reachables_field_item_spots, reachables_shops): (&[&'a Spot], &[&'a Shop]),
    (unreachable_field_item_spots, unreachable_shops): (&[&'a Spot], &[&'a Shop]),
) -> (Vec<Item>, Vec<Item>) {
    let remaining_spots: Vec<_> = unreachable_field_item_spots
        .iter()
        .copied()
        .chain(unreachable_shops.iter().map(|shop| &shop.spot))
        .collect();

    let (mut field_items, mut shop_items) =
        if let Some(priority_items) = items_pool.priority_items.take() {
            let (field_items, shop_items) = distribute_priority_items(
                rng,
                priority_items,
                reachables_field_item_spots.len(),
                reachables_shops
                    .iter()
                    .map(|shop| shop.count_general_items())
                    .sum(),
            );
            for _ in 0..shop_items.len() {
                items_pool
                    .field_items
                    .push(items_pool.shop_items.pop().unwrap());
            }
            (field_items, shop_items)
        } else {
            (Vec::new(), Vec::new())
        };

    // 少なくとも一つは行動を広げるアイテムを配置する
    let fi_spot_cnt = reachables_field_item_spots.len();
    let shop_display_cnt = reachables_shops
        .iter()
        .map(|shop| shop.count_general_items())
        .sum::<usize>();
    if field_items
        .iter()
        .chain(shop_items.iter())
        .any(|item| item.is_required(&remaining_spots))
    {
        let cnt = fi_spot_cnt - field_items.len();
        field_items.append(
            &mut items_pool
                .field_items
                .split_off(items_pool.field_items.len() - cnt),
        );
        let cnt = shop_display_cnt - shop_items.len();
        shop_items.append(
            &mut items_pool
                .shop_items
                .split_off(items_pool.shop_items.len() - cnt),
        );
    } else {
        let numerator = fi_spot_cnt as u32;
        let denominator = (fi_spot_cnt + shop_display_cnt) as u32;
        if rng.gen_ratio(numerator, denominator) {
            let cnt = fi_spot_cnt - field_items.len();
            let mut items = pick_items_include_requires(
                rng,
                &mut items_pool.field_items,
                &remaining_spots,
                cnt,
            );
            field_items.append(&mut items);
            let cnt = shop_display_cnt - shop_items.len();
            let mut items = items_pool
                .shop_items
                .split_off(items_pool.shop_items.len() - cnt);
            shop_items.append(&mut items);
        } else {
            let cnt = fi_spot_cnt - field_items.len();
            let mut items = items_pool
                .field_items
                .split_off(items_pool.field_items.len() - cnt);
            field_items.append(&mut items);
            let cnt = shop_display_cnt - shop_items.len();
            let mut items =
                pick_items_include_requires(rng, &mut items_pool.shop_items, &remaining_spots, cnt);
            shop_items.append(&mut items);
        };
    }
    field_items.shuffle(rng);
    shop_items.shuffle(rng);
    (field_items, shop_items)
}
