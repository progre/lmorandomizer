use std::collections::BTreeMap;

use anyhow::{Result, bail};

use crate::script::enums::FieldNumber;

use super::{
    files::{FieldYaml, RegionName},
    regions::RegionId,
};

/// region 一覧に積む順番と同じ順番で ID を振る
pub fn region_ids(fields: &[(FieldNumber, FieldYaml)]) -> Result<BTreeMap<RegionName, RegionId>> {
    let names: Vec<_> = fields
        .iter()
        .flat_map(|(_, field_yaml)| field_yaml.0.keys())
        .collect();
    let ids: BTreeMap<_, _> = names
        .iter()
        .enumerate()
        .map(|(index, &name)| (name.clone(), RegionId::new(index)))
        .collect();
    if ids.len() != names.len() {
        bail!("duplicate region name");
    }
    Ok(ids)
}
