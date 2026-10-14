/// `Storage` の spot の ID。shop は 1 枠ごとに振る
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpotId(usize);

impl SpotId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

/// item の ID。item が元々置かれていた spot の `SpotId` と同じ値を持つ
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ItemId(usize);

impl ItemId {
    /// item が元々置かれていた spot
    pub fn origin(self) -> SpotId {
        SpotId(self.0)
    }
}

impl From<SpotId> for ItemId {
    fn from(spot_id: SpotId) -> Self {
        Self(spot_id.0)
    }
}

/// `SpotId` を添字にした表。ROM を event に移した分などの欠番を持てる
pub struct SpotTable<T>(Vec<Option<T>>);

impl<T> SpotTable<T> {
    pub fn get(&self, id: SpotId) -> Option<&T> {
        self.0.get(id.0).and_then(|x| x.as_ref())
    }
}

impl<T> FromIterator<(SpotId, T)> for SpotTable<T> {
    fn from_iter<I: IntoIterator<Item = (SpotId, T)>>(iter: I) -> Self {
        let mut table = Vec::new();
        for (id, value) in iter {
            if table.len() <= id.0 {
                table.resize_with(id.0 + 1, || None);
            }
            table[id.0] = Some(value);
        }
        Self(table)
    }
}
