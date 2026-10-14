/// location の ID。location は、item を 1 つ置く場所である。
/// spot（YAML の 1 項目）とは別の概念である。
/// shop の spot は商品を最大 3 つ並べるので、商品 1 つごとに location が 1 つある
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LocationId(usize);

impl LocationId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

/// item の ID。種類ではなく、item の 1 個 1 個を区別する。
/// ID の値は、item が元々置かれていた location の `LocationId` と同じである。
/// そのため、同名の消耗品が複数あっても、それぞれ別の ID になる
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ItemId(usize);

impl From<LocationId> for ItemId {
    fn from(location_id: LocationId) -> Self {
        Self(location_id.0)
    }
}

/// `LocationId` を添字にした表。ROM を event に移した分などの欠番を持てる
pub struct LocationTable<T>(Vec<Option<T>>);

impl<T> LocationTable<T> {
    pub fn get(&self, id: LocationId) -> Option<&T> {
        self.0.get(id.0).and_then(|x| x.as_ref())
    }
}

impl<T> FromIterator<(LocationId, T)> for LocationTable<T> {
    fn from_iter<I: IntoIterator<Item = (LocationId, T)>>(iter: I) -> Self {
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
