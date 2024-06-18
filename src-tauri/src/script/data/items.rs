#[derive(Clone, Copy)]
#[repr(u8)]
pub enum _MainWeapon {
    Whip,
    ChainWhip,
    Mace,
    Knife,
    KeySword,
    Axe,
    Katana,
}

#[derive(Clone, Copy, num_derive::FromPrimitive)]
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

#[derive(Clone, Copy, num_derive::FromPrimitive)]
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

pub struct Rom(pub u8);
