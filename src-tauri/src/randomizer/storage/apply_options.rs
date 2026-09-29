use std::mem::take;

use crate::randomizer::RandomizeOptions;

use super::{Event, Storage};

/// シャッフルしない路傍の ROM は、元の場所で取得する event として扱う
pub fn apply_options(storage: &mut Storage, options: &RandomizeOptions) {
    log::trace!(
        "options.shuffle_secret_roms: {:?}",
        options.shuffle_secret_roms
    );
    if options.shuffle_secret_roms {
        return;
    }
    let roms = take(&mut storage.roms);
    storage.events.extend(roms.into_values().map(|x| Event {
        region: Some(x.spot.region().to_owned()),
        name: x.spot.name().to_owned().into(),
        requirements: Some(x.spot.requirements().to_owned()),
    }));
}
