use crate::dataset::spot::Region;

pub struct Regions<'a>(Vec<&'a Region>);

impl<'a> Regions<'a> {
    pub fn new(regions: Vec<&'a Region>) -> Regions<'a> {
        Regions(regions)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a Region> {
        self.0.iter().cloned()
    }
}
