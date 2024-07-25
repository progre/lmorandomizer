use std::{fmt::Display, str::FromStr};

use strum::ParseError;

#[derive(Clone, Copy, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum MainWeapon {
    Whip,
    ChainWhip,
    Mace,
    Knife,
    KeySword,
    Axe,
    Katana,
}

#[derive(Clone, Copy, PartialEq, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum SubWeapon {
    Shuriken = 0,
    Touken,
    Spear,
    FlareGun,
    Bomb,
    Pistol,
    Weights,
    AnkhJewel,
    Buckler,
    HandScanner,
    SilverShield,
    AngelShield,
    Ammunition,
}

#[derive(Clone, Copy, Debug, PartialEq, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum Equipment {
    /// 0 MSX
    Msx = 0,
    /// 1 ほら貝
    ShellHorn,
    /// 2 防水ケース
    WaterproofCase,
    /// 3 耐熱ケース
    HeatproofCase,
    /// 4 探知機
    Finder,
    /// 5 聖杯
    HolyGrail,
    /// 6 時のランプ
    LampOfTime,
    /// 7 防護服
    ProtectiveClothes,
    /// 8 お守り
    Talisman,
    /// 9 聖典
    Scriptures,
    /// 10 ガントレット
    Gauntlet,
    /// 11 リング
    Ring,
    /// 12 グローブ
    Glove,
    /// 13 無限の鍵
    KeyOfEternity,
    /// 14 双子の像
    TwinStatue,
    /// 15 銅鏡
    BronzeMirror,
    /// 16 ブーツ
    Boots,
    /// 17 羽
    Feather,
    /// 18 ブレスレット
    Bracelet,
    /// 19 龍の骨
    DragonBone,
    /// 20 かぎ爪
    GrappleClaw,
    /// 21 まが玉
    MagatamaJewel,
    /// 22 十字架
    Crucifix,
    /// 23 死者の書
    BookOfTheDead,
    /// 24 香水
    Perfume,
    /// 25 オカリナ
    Ocarina,
    /// 26 錨
    Anchor,
    /// 27 女性像
    WomanStatue,
    /// 28 小人人形
    MiniDoll,
    /// 29 真実の目
    EyeOfTruth,
    /// 30 ヘビの杖
    SerpentStaff,
    /// 31 氷のマント
    IceCape,
    /// 32 兜
    Helmet,
    /// 33 玉鱗
    Scalesphere,
    /// 34 水晶のドクロ
    CrystalSkull,
    /// 35 くさび
    DjedPillar,
    /// 36 飛行機模型
    PlaneModel,
    /// 37 はずみ車
    CogOfTheSoul,
    /// 38 ポシェットキー
    PochetteKey,
    /// 39 器
    Vessel,
    /// 40 MSX2
    Msx2,
    /// 41 日記
    Diary,
    /// 42 ムラーナの護符
    MulanaTalisman,
    /// 43 時のランプ（仕様済）
    LampOfTimeSpecified,
    /// 44 妊婦像
    MaternityStatue,
    /// 45 偽スキャナー
    FakeHandScanner,
    /// 46 コショウ
    Pepper,
    /// 47 財宝
    Treasures,
    /// 48 生命の薬（黄）
    MedicineOfLifeYellow,
    /// 49 生命の薬（緑）
    MedicineOfLifeGreen,
    /// 50 生命の薬（赤）
    MedicineOfLifeRed,
    /// 51 白銀の盾（偽物）
    FakeSilverShield,
    /// 52 ラ・ムラーナの秘宝
    TheTreasuresOfLaMurana,
    /// 53 生命の宝珠
    SacredOrb,
    /// 54 地図
    Map,
    /// 55 始まりの印
    OriginSeal,
    /// 56 誕生の印
    BirthSeal,
    /// 57 営みの印
    LifeSeal,
    /// 58 滅びの印
    DeathSeal,
    /// 59 あぶねぇ水着
    SweetClothing,
    /// 100 10倍カードリッジ
    GameMaster = 100,
}

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

impl Rom {
    pub fn try_from_camel_case(camel_case: &str) -> Result<Self, ParseError> {
        let pascal_case_name: String = camel_case[0..1]
            .to_uppercase()
            .chars()
            .chain(camel_case[1..].chars())
            .collect();
        Rom::from_str(&pascal_case_name)
    }
}

impl Display for Rom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum Seal {
    Origin,
    Birth,
    Life,
    Death,
}
