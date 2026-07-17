mod items;

use std::mem::take;

use rand::Rng;

use crate::randomizer::storage::item::Item;

pub use items::{ShuffledItems, UnorderedItems};

/// 棄却再抽選の上限。進行候補を引ける状態なら 1 回あたりの成功確率は
/// 最低でも 1/(pool サイズ) 程度あるため、この回数で失敗することは実質ない。
const MAX_RETRY_COUNT: usize = 10_000;

#[derive(Clone)]
pub struct ItemsPool<'a> {
    pub priority_items: Option<UnorderedItems<'a>>,
    pub field_items: ShuffledItems<'a>,
    pub talk_items: ShuffledItems<'a>,
    pub shop_items: ShuffledItems<'a>,
    pub consumable_items: UnorderedItems<'a>,
}

impl<'a> ItemsPool<'a> {
    pub fn move_shop_items_to_field_items(&mut self, rng: &mut impl Rng, cnt: usize) {
        self.field_items = take(&mut self.field_items)
            .append_count(&mut self.shop_items, cnt)
            .shuffle(rng);
    }

    /// 通常の一様抽選。呼ぶたびに pool を再シャッフルするため、
    /// 各呼び出しは独立した一様抽選になる。
    fn pick_items_randomly(
        &mut self,
        rng: &mut impl Rng,
        field_count: usize,
        talk_count: usize,
        shop_count: usize,
    ) -> (ShuffledItems<'a>, ShuffledItems<'a>, ShuffledItems<'a>) {
        debug_assert!(self.priority_items.is_none());
        self.field_items.shuffle(rng);
        self.talk_items.shuffle(rng);
        self.shop_items.shuffle(rng);
        (
            self.field_items
                .split_off(self.field_items.len() - field_count),
            self.talk_items
                .split_off(self.talk_items.len() - talk_count),
            self.shop_items
                .split_off(self.shop_items.len() - shop_count),
        )
    }

    /// 「進行候補を少なくとも1つ含む」集合になるまで一様抽選を繰り返すことで、
    /// 条件付き一様な抽選を行う。棄却は一時 pool 上で行い、採用時のみ本体へ反映する。
    ///
    /// 進行候補をそもそも引けない場合(最終 sphere や、単体では進行しない
    /// アイテムの組み合わせしか残っていない場合)は、条件なしの一様抽選を返す。
    pub fn pick_items_with_retry(
        &mut self,
        rng: &mut impl Rng,
        field_count: usize,
        talk_count: usize,
        shop_count: usize,
        is_progression: impl Fn(&Item) -> bool,
    ) -> Option<(ShuffledItems<'a>, ShuffledItems<'a>, ShuffledItems<'a>)> {
        let pools = [
            (field_count, &self.field_items),
            (talk_count, &self.talk_items),
            (shop_count, &self.shop_items),
        ];
        let pickable = pools.into_iter().any(|(count, pool)| {
            count > 0 && pool.as_slice().iter().any(|item| is_progression(item))
        });
        if !pickable {
            return Some(self.pick_items_randomly(rng, field_count, talk_count, shop_count));
        }
        for _ in 0..MAX_RETRY_COUNT {
            let mut candidate_pool = self.clone();
            let picked =
                candidate_pool.pick_items_randomly(rng, field_count, talk_count, shop_count);
            let contains_progression = [&picked.0, &picked.1, &picked.2]
                .into_iter()
                .flat_map(|items| items.as_slice())
                .any(|item| is_progression(item));
            if contains_progression {
                *self = candidate_pool;
                return Some(picked);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        randomizer::{
            spoiler::make_rng,
            storage::item::{Item, StrategyFlag},
        },
        script::enums::MainWeapon,
    };

    use super::*;

    fn item(name: &str) -> Item {
        Item::main_weapon(MainWeapon::Whip, StrategyFlag::new(name.to_owned()))
    }

    fn items_pool<'a>(rng: &mut impl Rng, field_items: Vec<&'a Item>) -> ItemsPool<'a> {
        ItemsPool {
            priority_items: None,
            field_items: UnorderedItems::new(field_items).shuffle(rng),
            talk_items: Default::default(),
            shop_items: Default::default(),
            consumable_items: Default::default(),
        }
    }

    /// 進行候補 A, B と非進行候補 x, y, z から2個選ぶとき、
    /// 「進行候補を少なくとも1つ含む」7通りが概ね同じ頻度になること。
    /// 旧方式(進行候補を1つ先に投入)では AB が過大になる。
    #[test]
    fn test_pick_items_with_retry_is_conditionally_uniform() {
        let items: Vec<_> = ["objA", "objB", "objX", "objY", "objZ"]
            .iter()
            .map(|name| item(name))
            .collect();
        let is_progression = |item: &Item| matches!(item.name.get(), "objA" | "objB");

        let mut rng = make_rng("test");
        let mut counts: BTreeMap<String, u32> = BTreeMap::new();
        const TRIALS: u32 = 7000;
        for _ in 0..TRIALS {
            let mut pool = items_pool(&mut rng, items.iter().collect());
            let (field, talk, shop) = pool
                .pick_items_with_retry(&mut rng, 2, 0, 0, is_progression)
                .unwrap();
            assert_eq!(field.len(), 2);
            assert_eq!(talk.len(), 0);
            assert_eq!(shop.len(), 0);
            assert_eq!(pool.field_items.len(), 3);
            let mut names: Vec<_> = field.as_slice().iter().map(|x| x.name.get()).collect();
            names.sort();
            *counts.entry(names.join("+")).or_default() += 1;
        }

        // 進行候補を含まない組み合わせ(xy, xz, yz)は棄却されている
        assert_eq!(counts.len(), 7);
        assert!(
            counts
                .keys()
                .all(|key| key.contains("objA") || key.contains("objB"))
        );
        // 条件付き一様: 各組み合わせが期待値 1000 の ±10% に収まる
        let expected = TRIALS / 7;
        for (key, count) in &counts {
            assert!(
                (expected * 9 / 10..=expected * 11 / 10).contains(count),
                "{}: {} (expected around {})",
                key,
                count,
                expected
            );
        }
    }

    /// 進行候補を引けない場合は条件なしの一様抽選にフォールバックする
    #[test]
    fn test_pick_items_with_retry_without_progression_candidates() {
        let items: Vec<_> = ["objX", "objY", "objZ"]
            .iter()
            .map(|name| item(name))
            .collect();
        let mut rng = make_rng("test");
        let mut pool = items_pool(&mut rng, items.iter().collect());
        let picked = pool.pick_items_with_retry(&mut rng, 2, 0, 0, |_| false);
        assert_eq!(picked.unwrap().0.len(), 2);
        assert_eq!(pool.field_items.len(), 1);
    }
}
