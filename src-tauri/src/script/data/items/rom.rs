use std::fmt;

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive, strum::EnumString,
)]
#[repr(u8)]
pub enum Rom {
    /// 10倍カートリッジ
    GameMaster,
    /// 新10倍カートリッジ
    GameMaster2,
    /// 古文書リーダー
    GlyphReader,
    /// 遺跡RAM8K
    RuinsRam8K,
    /// 遺跡RAM16K
    RuinsRam16K,
    /// 開発中止ROM
    #[allow(clippy::enum_variant_names)]
    UnreleasedRom,
    /// PR3
    Pr3,
    /// GR3
    Gr3,
    /// わんぱくアスレチック
    AthleticLand,
    /// けっきょく南極大冒険
    AntarcticAdventure,
    /// モン太君のいち・に・さんすう
    MonkeyAcademy,
    /// タイムパイロット
    TimePilot,
    /// フロッガー
    Frogger,
    /// スーパーコブラ
    SuperCobra,
    /// ビデオハスラー
    VideoHustler,
    /// コナミの麻雀道場
    MahjongDojo,
    /// ハイパーオリンピック1
    HyperOlympic1,
    /// ハイパーオリンピック2
    HyperOlympic2,
    /// ハイパーオリンピック3
    HyperOlympic3,
    /// サーカスチャーリー
    CircusCharlie,
    /// マジカルツリー
    MagicalTree,
    /// ぽんぽこパン
    ComicBakery,
    /// ハイパースポーツ1
    HyperSports1,
    /// ハイパースポーツ2
    HyperSports2,
    /// ハイパースポーツ3
    HyperSports3,
    /// キャベッジパッチキッズ
    CabbagePatchKids,
    /// ハイパーラリー
    HyperRally,
    /// コナミのテニス
    KonamiTennis,
    /// スカイジャガー
    SkyJaguar,
    /// コナミのピンボール
    KonamiPinball,
    /// コナミのゴルフ
    KonamiGolf,
    /// コナミのベースボール
    KonamiBaseball,
    /// イーアルカンフー
    YieArKungFu,
    /// 王家の谷
    KingsValley,
    /// モピレンジャー
    MopiRanger,
    /// ピポルス
    Pippols,
    /// ロードファイター
    RoadFighter,
    /// コナミのピンポン
    KonamiPingPong,
    /// コナミのサッカー
    KonamiSoccer,
    /// グーニーズ
    Goonies,
    /// コナミのボクシング
    KonamiBoxing,
    /// イーガー皇帝の逆襲
    YieArKungFu2,
    /// 魔城伝説
    Knightmare,
    /// ツインビー
    Twinbee,
    /// 新世サイザー
    ShinSynthesizer,
    /// グラディウス
    Gradius,
    /// 夢大陸アドベンチャー
    PenguinAdventure,
    /// 悪魔城ドラキュラ
    Castlevania,
    /// キングコング2 蘇る伝説
    KingKong2,
    /// Qバート
    QBert,
    /// 火の鳥
    Firebird,
    /// がんばれゴエモン!からくり道中
    GanbareGoemon,
    /// ガリウスの迷宮
    MazeOfGalious,
    /// メタルギア
    MetalGear,
    /// グラディウス2
    Gradius2,
    /// F1スピリット
    F1Spirit,
    /// ウシャス
    Usas,
    /// シャロム
    Shalom,
    /// ブレイクショット
    BreakShot,
    /// 激突ペナントレース
    PennantRace,
    /// 沙羅曼蛇
    Salamander,
    /// パロディウス
    Parodius,
    /// エルギーザの封印
    SealOfElGiza,
    /// 魂斗羅
    Contra,
    /// 天と地と
    HeavenAndEarth,
    /// ゴーファーの野望 エピソードII
    Nemesis3,
    /// 牌の魔術師
    MahjongWizard,
    /// 激突ペナントレース2
    PennantRace2,
    /// メタルギア2 ソリッドスネーク
    MetalGear2,
    /// スペースマンボウ
    SpaceManbow,
    /// クォース
    Quarth,
    /// 王家の谷 ディスク版
    KingsValleyDisk,
    /// コナミの占いセンセーション
    DivinerSensation,
    /// スナッチャー
    Snatcher,
    /// F1スピリット 3Dスペシャル
    F1Spirit3D,
    /// コナミゲームコレクション1
    GameCollection1,
    /// コナミゲームコレクション2
    GameCollection2,
    /// コナミゲームコレクション3
    GameCollection3,
    /// コナミゲームコレクション4
    GameCollection4,
    /// コナミゲームコレクション番外編
    GameCollectionEx,
    /// SDスナッチャー
    SdSnatcher,
    /// バッドランズ
    Badlands,
    /// グラディウス2 ベータ
    Gradius2Beta,
    /// A1スピリット
    A1Spirit,
}

impl fmt::Display for Rom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
