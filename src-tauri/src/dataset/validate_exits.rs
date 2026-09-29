use std::collections::{BTreeMap, BTreeSet};

use log::trace;

use super::{files::RegionName, spot::Region};

pub fn validate_exits(regions: &[Region]) {
    let all_region_names: BTreeSet<&RegionName> = regions.iter().map(|r| r.name()).collect();

    // region name -> 自分へ向かってくる exit 元 region 名の集合
    let exits_to: BTreeMap<&RegionName, BTreeSet<&RegionName>> = regions
        .iter()
        .map(|r| {
            let targets: BTreeSet<_> = r.exits().iter().map(|exit| &exit.target).collect();
            (r.name(), targets)
        })
        .collect();

    for region in regions {
        for exit in region.exits() {
            let (dir, target_name) = (exit.direction, &exit.target);
            if !all_region_names.contains(target_name) {
                trace!(
                    "[validate] exit target not found: {} -({})-> {}",
                    region.name().get(),
                    dir,
                    target_name.get()
                );
                continue;
            }
            let target_exits = &exits_to[target_name];
            if !target_exits.contains(region.name()) {
                trace!(
                    "[validate] no return exit: {} -({})-> {} (no exit back)",
                    region.name().get(),
                    dir,
                    target_name.get()
                );
            }
        }
    }
}
