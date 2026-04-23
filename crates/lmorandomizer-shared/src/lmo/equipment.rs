use std::{fmt, num::NonZero};

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    num_derive::FromPrimitive,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIter,
    strum::EnumString,
)]
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
}

impl Equipment {
    #[inline(always)]
    pub fn related_flag(&self) -> Option<NonZero<u16>> {
        match self {
            Equipment::Msx => None,
            Equipment::ShellHorn => Some(753.try_into().unwrap()),
            Equipment::WaterproofCase => Some(792.try_into().unwrap()),
            Equipment::HeatproofCase => Some(751.try_into().unwrap()),
            Equipment::Finder => Some(754.try_into().unwrap()),
            Equipment::HolyGrail => Some(755.try_into().unwrap()),
            Equipment::LampOfTime => Some(756.try_into().unwrap()),
            Equipment::ProtectiveClothes => Some(757.try_into().unwrap()),
            Equipment::Talisman => Some(758.try_into().unwrap()),
            Equipment::Scriptures => Some(761.try_into().unwrap()),
            Equipment::Gauntlet => Some(762.try_into().unwrap()),
            Equipment::Ring => Some(763.try_into().unwrap()),
            Equipment::Glove => Some(764.try_into().unwrap()),
            Equipment::KeyOfEternity => Some(765.try_into().unwrap()),
            Equipment::TwinStatue => Some(766.try_into().unwrap()),
            Equipment::BronzeMirror => Some(767.try_into().unwrap()),
            Equipment::Boots => Some(768.try_into().unwrap()),
            Equipment::Feather => Some(769.try_into().unwrap()),
            Equipment::Bracelet => Some(770.try_into().unwrap()),
            Equipment::DragonBone => Some(771.try_into().unwrap()),
            Equipment::GrappleClaw => Some(772.try_into().unwrap()),
            Equipment::MagatamaJewel => Some(773.try_into().unwrap()),
            Equipment::Crucifix => Some(774.try_into().unwrap()),
            Equipment::BookOfTheDead => Some(775.try_into().unwrap()),
            Equipment::Perfume => Some(776.try_into().unwrap()),
            Equipment::Ocarina => Some(777.try_into().unwrap()),
            Equipment::Anchor => Some(778.try_into().unwrap()),
            Equipment::WomanStatue => Some(779.try_into().unwrap()),
            Equipment::MiniDoll => Some(780.try_into().unwrap()),
            Equipment::EyeOfTruth => Some(781.try_into().unwrap()),
            Equipment::SerpentStaff => Some(782.try_into().unwrap()),
            Equipment::IceCape => Some(783.try_into().unwrap()),
            Equipment::Helmet => Some(784.try_into().unwrap()),
            Equipment::Scalesphere => Some(785.try_into().unwrap()),
            Equipment::CrystalSkull => Some(786.try_into().unwrap()),
            Equipment::DjedPillar => Some(787.try_into().unwrap()),
            Equipment::PlaneModel => Some(788.try_into().unwrap()),
            Equipment::CogOfTheSoul => Some(789.try_into().unwrap()),
            Equipment::PochetteKey => Some(790.try_into().unwrap()),
            Equipment::Vessel => Some(791.try_into().unwrap()),
            Equipment::Msx2 => Some(752.try_into().unwrap()),
            Equipment::Diary => Some(759.try_into().unwrap()),
            Equipment::MulanaTalisman => Some(760.try_into().unwrap()),
            Equipment::LampOfTimeSpecified => Some(756.try_into().unwrap()),
            Equipment::MaternityStatue => Some(779.try_into().unwrap()),
            Equipment::FakeHandScanner => Some(494.try_into().unwrap()),
            Equipment::Pepper => Some(793.try_into().unwrap()),
            Equipment::Treasures => Some(794.try_into().unwrap()),
            Equipment::MedicineOfLifeYellow => Some(625.try_into().unwrap()),
            Equipment::MedicineOfLifeGreen => Some(626.try_into().unwrap()),
            Equipment::MedicineOfLifeRed => Some(627.try_into().unwrap()),
            Equipment::FakeSilverShield => Some(524.try_into().unwrap()),
            Equipment::TheTreasuresOfLaMurana => None,
            Equipment::SacredOrb => None,
            Equipment::Map => None,
            Equipment::OriginSeal => Some(795.try_into().unwrap()),
            Equipment::BirthSeal => Some(796.try_into().unwrap()),
            Equipment::LifeSeal => Some(797.try_into().unwrap()),
            Equipment::DeathSeal => Some(798.try_into().unwrap()),
            Equipment::SweetClothing => None,
        }
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
