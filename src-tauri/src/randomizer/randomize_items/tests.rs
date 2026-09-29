use std::collections::HashMap;

use sha3::Digest;

use crate::{
    app::read_game_structure_files_debug,
    randomizer::storage::{apply_options::apply_options, create_source::create_source, item::Item},
};

use super::*;

fn spot_items(storage: &Storage) -> impl Iterator<Item = (String, &Item)> {
    storage
        .main_weapons
        .values()
        .map(|x| (x.spot.to_string(), &x.item))
        .chain(
            storage
                .sub_weapons
                .values()
                .map(|x| (x.spot.to_string(), &x.item)),
        )
        .chain(
            storage
                .chests
                .values()
                .map(|x| (x.spot.to_string(), &x.item)),
        )
        .chain(
            storage
                .seals
                .values()
                .map(|x| (x.spot.to_string(), &x.item)),
        )
        .chain(storage.roms.values().map(|x| (x.spot.to_string(), &x.item)))
        .chain(storage.talks.iter().map(|x| (x.spot.to_string(), &x.item)))
        .chain(
            storage
                .shops
                .iter()
                .map(|x| (format!("{}[{}]", x.spot, x.idx), &x.item)),
        )
}

/// 配置結果を `spot = item 名 (item が元々あった spot)` の行で表す。
/// `Storage` の構造や並び順を変えても同じ配置なら同じテキストになるよう、表示名だけで組み立ててソートする。
/// 同名の消耗品が別の枠と入れ替わったことも検出するため、item の元の spot も書く。
fn placement_text(source: &Storage, shuffled: &Storage) -> String {
    let origins: HashMap<_, _> = spot_items(source)
        .map(|(spot, item)| (format!("{:?}", item.src), spot))
        .collect();
    assert_eq!(origins.len(), spot_items(source).count());

    let mut lines: Vec<_> = spot_items(shuffled)
        .map(|(spot, item)| {
            let origin = &origins[&format!("{:?}", item.src)];
            format!("{spot} = {} ({origin})", item.name.get())
        })
        .collect();
    lines.sort();
    let line_count = lines.len();
    lines.dedup();
    assert_eq!(lines.len(), line_count);
    lines.join("\n")
}

#[tokio::test]
async fn test_shuffle_hash() -> Result<()> {
    let game_structure = read_game_structure_files_debug().await?;
    let opts = RandomizeOptions {
        seed: "test".to_owned(),
        shuffle_secret_roms: true,
        need_glitches: false,
        absolutely_shuffle: false,
    };
    let mut source = create_source(&game_structure)?;
    apply_options(&mut source, &opts);
    let (shuffled, spoiler_log) = shuffle(&source, &opts);

    let placement_str = placement_text(&source, &shuffled);
    let placement_hash = hex::encode(sha3::Sha3_512::digest(placement_str));
    const EXPECTED_PLACEMENT_HASH: &str = "1bc118059e4413924cd70eb92cf86ce0dd844d9111618b6b7434228d81bf25062cd72213c35b530f578cb1da33dfefb28aa488f52d9795a56bd95f4ecf3fad2d";
    assert_eq!(placement_hash, EXPECTED_PLACEMENT_HASH);

    let spoiler_log_str = format!("{}", spoiler_log.to_owned());
    let spoiler_log_hash = hex::encode(sha3::Sha3_512::digest(spoiler_log_str));
    const EXPECTED_SPOILER_LOG_HASH: &str = "a785f4bf760e45c53cf76992a75fd5dbd986e0299a9438c738cbcb21cb75333b340ba781cb480f7f8f232accddc5d92e489bb87621d2c5819da616f688f16200";
    assert_eq!(spoiler_log_hash, EXPECTED_SPOILER_LOG_HASH);

    Ok(())
}

#[tokio::test]
async fn test_shuffle_multi_patterns() -> Result<()> {
    let game_structure = read_game_structure_files_debug().await?;
    for i in 0..100 {
        let opts = RandomizeOptions {
            seed: i.to_string(),
            shuffle_secret_roms: true,
            need_glitches: true,
            absolutely_shuffle: false,
        };
        let mut source = create_source(&game_structure)?;
        apply_options(&mut source, &opts);
        let (_, spoiler_log) = shuffle(&source, &opts);
        assert_eq!(
            spoiler_log.count_checkpoints(),
            source.all_items().count() + source.events.len()
        );
    }

    Ok(())
}
