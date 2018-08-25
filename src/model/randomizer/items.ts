export const fields = {
  surface: 'surface',
  gateOfGuidance: 'gateOfGuidance',
  mausoleumOfTheGiants: 'mausoleumOfTheGiants',
  templeOfTheSun: 'templeOfTheSun',
  springInTheSky: 'springInTheSky',
  infernoCavern: 'infernoCavern',
  chamberOfExtinction: 'chamberOfExtinction',
  twinLabyrinthsFront: 'twinLabyrinthsFront',
  endlessCorridor: 'endlessCorridor',
  shrineOfTheMother: 'shrineOfTheMother',
  gateOfIllusion: 'gateOfIllusion',
  graveyardOfTheGiants: 'graveyardOfTheGiants',
  templeOfMoonlight: 'templeOfMoonlight',
  towerOfTheGoddess: 'towerOfTheGoddess',
  towerOfRuin: 'towerOfRuin',
  chamberOfBirth: 'chamberOfBirth',
  twinLabyrinthsBack: 'twinLabyrinthsBack',
  dimensionalCorridor: 'dimensionalCorridor',
  trueShrineOfTheMother: 'trueShrineOfTheMother',
  hellTemple: 'hellTemple',
};
type Field = keyof typeof fields;

export const CHESTS: ReadonlyArray<Readonly<{
  source: string;
  requiredObjectivesList: ReadonlyArray<ReadonlyArray<Field | Item>>;
}>> = (() => {
  const list = [];
  const array = [
    // 導きの門
    '<OBJECT 1,32768,30720,60,47,794,-1>', [], // 財宝
    '<OBJECT 1,30720,6144,41,53,717,-1>', [], // 生命の宝珠
    '<OBJECT 1,49152,38912,42,22,774,-1>', null,
    '<OBJECT 1,47104,22528,43,54,727,-1>', null,
    '<OBJECT 1,2048,30720,44,101,801,-1>', null,
    '<OBJECT 1,45056,14336,62,5,755,-1>', null,
    '<OBJECT 1,2048,22528,107,17,769,-1>', null,
    '<OBJECT 1,2048,14336,102,1,753,-1>', null,
    // '<OBJECT 1,53248,22528,1039,-1,1039,-1>', null, 印
    '<OBJECT 1,45056,30720,106,53,725,-1>', null,
    '<OBJECT 1,36864,6144,142,54,728,-1>', null,
    // '<OBJECT 1,57344,14336,143,-1,744,-1>', null, アンクジュエル
    '<OBJECT 1,24576,14336,144,53,718,-1>', null,
    '<OBJECT 1,22528,30720,185,15,767,-1>', null,
    '<OBJECT 1,59392,30720,180,54,729,-1>', null,
    '<OBJECT 1,57344,30720,222,8,758,-1>', null,
    '<OBJECT 1,43008,22528,182,16,768,-1>', null,
    '<OBJECT 1,36864,30720,183,53,719,-1>', null,
    // '<OBJECT 1,2048,6144,184,-1,745,-1>', null, 太陽神殿アンクジュエル
    '<OBJECT 1,30720,6144,241,53,720,-1>', null,
    '<OBJECT 1,14336,6144,243,12,764,-1>', null,
    '<OBJECT 1,30720,6144,242,54,730,-1>', null,
    '<OBJECT 1,2048,14336,245,33,785,-1>', null,
    // '<OBJECT 1,4096,14336,244,-1,795,-1>', null, 印
    // '<OBJECT 1,59392,6144,246,-1,746,-1>', null, アンクジュエル
    '<OBJECT 1,2048,14336,283,54,731,-1>', null,
    '<OBJECT 1,59392,6144,281,20,772,-1>', null,
    '<OBJECT 1,2048,6144,320,53,722,-1>', null,
    '<OBJECT 1,43008,22528,321,54,732,-1>', null,
    // '<OBJECT 1,59392,22528,322,-1,797,-1>', null, 印
    '<OBJECT 1,2048,22528,400,54,733,-1>', null,
    '<OBJECT 1,8192,6144,419,14,766,-1>', null,
    '<OBJECT 1,8192,6144,420,14,766,0>', null,
    '<OBJECT 1,6144,30720,453,34,786,-1>', null,
    '<OBJECT 1,30720,38912,452,53,724,-1>', null,
    '<OBJECT 1,59392,6144,451,41,759,-1>', null,
    '<OBJECT 1,30720,14336,454,54,734,-1>', null,
    // '<OBJECT 1,30720,14336,455,-1,798,-1>', null, 印
    '<OBJECT 1,30720,6144,481,11,763,-1>', null,
    '<OBJECT 1,55296,14336,482,54,735,-1>', null,
    '<OBJECT 1,18432,28672,483,53,723,-1>', null,
    // '<OBJECT 1,28672,10240,490,-1,749,-1>', null, アンクジュエル
    '<OBJECT 1,18432,30720,478,13,765,-1>', null,
    '<OBJECT 1,2048,14336,522,7,757,-1>', null,
    '<OBJECT 1,26624,14336,521,54,736,-1>', null,
    '<OBJECT 1,32768,30720,523,37,789,-1>', null,
    '<OBJECT 1,43008,14336,40,163,863,-1>', null,
    '<OBJECT 1,38912,6144,542,54,737,-1>', null,
    '<OBJECT 1,14336,6144,543,10,762,-1>', null,
    '<OBJECT 1,2048,38912,900,183,883,-1>', null,
    '<OBJECT 1,32768,28672,891,36,788,-1>', null,
    '<OBJECT 1,57344,38912,896,54,738,-1>', null,
    '<OBJECT 1,59392,38912,893,145,845,-1>', null,
    '<OBJECT 1,36864,22528,894,29,781,-1>', null,
    '<OBJECT 1,38912,38912,564,23,775,-1>', null,
    '<OBJECT 1,30720,22528,560,31,783,-1>', null,
    '<OBJECT 1,59392,22528,561,54,739,-1>', null,
    // '<OBJECT 1,43008,22528,10,-1,-1,-1>', null, 手前より開け
    // '<OBJECT 1,47104,22528,11,-1,-1,-1>', null,
    // '<OBJECT 1,51200,22528,12,-1,-1,-1>', null,
    // '<OBJECT 1,55296,22528,13,-1,-1,-1>', null,
    '<OBJECT 1,30720,30720,562,25,777,-1>', null,
    '<OBJECT 1,51200,6144,563,30,782,-1>', null,
    '<OBJECT 1,28672,6144,593,35,787,-1>', null,
    '<OBJECT 1,34816,14336,594,54,740,-1>', null,
    // '<OBJECT 1,30720,30720,592,-1,747,-1>', null, アンクジュエル
    '<OBJECT 1,34816,14336,591,53,721,-1>', null,
    '<OBJECT 1,57344,38912,619,39,791,-1>', null,
    '<OBJECT 1,57344,14336,610,24,776,-1>', null,
    '<OBJECT 1,6144,6144,611,27,779,-1>', null,
    '<OBJECT 1,32768,38912,613,54,741,-1>', null,
    '<OBJECT 1,57344,14336,612,38,790,-1>', null,
    '<OBJECT 1,8192,38912,716,21,773,-1>', null,
    '<OBJECT 1,14336,24576,922,53,726,-1>', null,
    // '<OBJECT 1,59392,6144,920,-1,750,-1>', null, アンクジュエル
    '<OBJECT 1,2048,6144,920,54,742,-1>', null,
    '<OBJECT 1,30720,28672,343,59,344,-1>', null,
    '<OBJECT 1,2048,22528,107,17,769,-1>', null,
    '<OBJECT 1,2048,14336,102,1,753,-1>', null,
    // '<OBJECT 1,53248,22528,1039,-1,1039,-1>', null, 夜 誕生の印
    '<OBJECT 1,45056,30720,106,53,725,-1>', null,
  ];
  for (let i = 0; i < array.length; i += 2) {
    list.push({
      source: <string>array[i],
      requiredObjectivesList: <(Field | Item)[][]><any>array[i + 1],
    });
  }
  return list;
})();

export enum Item {
  msx,
  conch,
  waterproofCase,
  heatproofCase,
  finder,
  holyGrail,
  timeLamp,
  protectiveClothing,
  amulet,
  scripture,
  gauntlet,
  ring,
  glove,
  infiniteKey,
  twinStatue,
  copperMirror,
  boots,
  feather,
  bracelet,
  dragonSBone,
  grappleClaw,
  magomeball,
  cross,
  bookOfTheDead,
  perfume,
  ocarina,
  anchor,
  femaleImage,
  dwarf,
  theEyesOfTruth,
  snakeSCane,
  iceCloak,
  helmet,
  grainScale,
  crystalSkull,
  wedge,
  airplaneModel,
  flyWheel,
  pochetteKey,
  vessel,
  msx2,
  diary,
  amuletOfMurana,
  hourLampSpecified,
  pregnantWoman,
  fakeScanner,
  pepper,
  treasures,
  medicineOfLifeYellow,
  medicineOfLifeGreen,
  medicineOfLifeRed,
  silverShieldFake,
  theTreasuresOfLaMurana,
  sacredOrb,
  map,
  markOfBeginning,
  signsOfBirth,
  markOfBusiness,
  markOfDestruction,
  sweetClothing,
}

//  0 MSX
//  1 ほら貝
//  2 防水ケース
//  3 耐熱ケース
//  4 探知機
//  5 聖杯
//  6 時のランプ
//  7 防護服
//  8 お守り
//  9 聖典
// 10 ガントレット
// 11 リング
// 12 グローブ
// 13 無限の鍵
// 14 双子の像
// 15 銅鏡
// 16 ブーツ
// 17 羽
// 18 ブレスレット
// 19 龍の骨
// 20 かぎ爪
// 21 まが玉
// 22 十字架
// 23 死者の書
// 24 香水
// 25 オカリナ
// 26 錨
// 27 女性像
// 28 小人人形
// 29 真実の目
// 30 ヘビの杖
// 31 氷のマント
// 32 兜
// 33 玉鱗
// 34 水晶のドクロ
// 35 くさび
// 36 飛行機模型
// 37 はずみ車
// 38 ポシェットキー
// 39 器
// 40 MSX2
// 41 日記
// 42 ムラーナの護符
// 43 時のランプ（仕様済）
// 44 妊婦像
// 45 偽スキャナー
// 46 コショウ
// 47 財宝
// 48 生命の薬（黄）
// 49 生命の薬（緑）
// 50 生命の薬（赤）
// 51 白銀の盾（偽物）
// 52 ラ・ムラーナの秘宝
// 53 生命の宝珠
// 54 地図
// 55 始まりの印
// 56 誕生の印
// 57 営みの印
// 58 滅びの印
// 59 あぶねぇ水着
