use sha3::Digest;

use crate::{
    app::read_game_structure_files_debug, randomizer::storage::create_source::create_source,
};

use super::*;

#[tokio::test]
async fn test_shuffle_hash() -> Result<()> {
    let game_structure = read_game_structure_files_debug().await?;
    let opts = RandomizeOptions {
        seed: "test".to_owned(),
        shuffle_secret_roms: true,
        need_glitches: false,
        absolutely_shuffle: false,
    };
    let source = create_source(&game_structure, &opts)?;
    let (shuffled, spoiler_log) = shuffle(&source, &opts);

    let shuffled_str = format!("{:?}", shuffled);
    let shuffled_hash = hex::encode(sha3::Sha3_512::digest(shuffled_str));
    const EXPECTED_SHUFFLED_HASH: &str = "08db6ede6565f6888eb3ea4cd77a4b51c96200aea6664dd8eeab292147af1a666c99432234e0e1da0938a9001f00b1ba1d12def0aec6a47e340c15c014c96b80";
    assert_eq!(shuffled_hash, EXPECTED_SHUFFLED_HASH);

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
        let source = create_source(&game_structure, &opts)?;
        let (_, spoiler_log) = shuffle(&source, &opts);
        assert_eq!(
            spoiler_log.count_checkpoints(),
            source.all_items().count() + source.events.len()
        );
    }

    Ok(())
}
