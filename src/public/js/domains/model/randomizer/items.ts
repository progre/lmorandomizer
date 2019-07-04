export type MainWeaponNumber = mainWeaponNumbers;
export enum mainWeaponNumbers {
  whip,
  chainWhip,
  mace,
  knife,
  keySword,
  axe,
  katana,
}

export type SubWeaponNumber = subWeaponNumbers;
export enum subWeaponNumbers {
  shuriken,
  touken,
  spear,
  flareGun,
  bomb,
  pistol,
  weights,
  ankhJewel,
  buckler,
  handScanner,
  silverShield,
  angelShield,
  ammunition,
}

export type EquipmentNumber = equipmentNumbers;
export enum equipmentNumbers {
  msx, // 0 MSX
  shellHorn, // 1 ほら貝
  waterproofCase, // 2 防水ケース
  heatproofCase, // 3 耐熱ケース
  finder, // 4 探知機
  holyGrail, // 5 聖杯
  lampOfTime, // 6 時のランプ
  protectiveClothes, // 7 防護服
  talisman, // 8 お守り
  scriptures, // 9 聖典
  gauntlet, // 10 ガントレット
  ring, // 11 リング
  glove, // 12 グローブ
  keyOfEternity, // 13 無限の鍵
  twinStatue, // 14 双子の像
  bronzeMirror, // 15 銅鏡
  boots, // 16 ブーツ
  feather, // 17 羽
  bracelet, // 18 ブレスレット
  dragonBone, // 19 龍の骨
  grappleClaw, // 20 かぎ爪
  magatamaJewel, // 21 まが玉
  crucifix, // 22 十字架
  bookOfTheDead, // 23 死者の書
  perfume, // 24 香水
  ocarina, // 25 オカリナ
  anchor, // 26 錨
  womanStatue, // 27 女性像
  miniDoll, // 28 小人人形
  eyeOfTruth, // 29 真実の目
  serpentStaff, // 30 ヘビの杖
  iceCape, // 31 氷のマント
  helmet, // 32 兜
  scalesphere, // 33 玉鱗
  crystalSkull, // 34 水晶のドクロ
  djedPillar, // 35 くさび
  planeModel, // 36 飛行機模型
  cogOfTheSoul, // 37 はずみ車
  pochetteKey, // 38 ポシェットキー
  vessel, // 39 器
  msx2, // 40 MSX2
  diary, // 41 日記
  mulanaTalisman, // 42 ムラーナの護符
  lampOfTimeSpecified, // 43 時のランプ（仕様済）
  maternityStatue, // 44 妊婦像
  fakeHandScanner, // 45 偽スキャナー
  pepper, // 46 コショウ
  treasures, // 47 財宝
  medicineOfLifeYellow, // 48 生命の薬（黄）
  medicineOfLifeGreen, // 49 生命の薬（緑）
  medicineOfLifeRed, // 50 生命の薬（赤）
  fakeSilverShield, // 51 白銀の盾（偽物）
  theTreasuresOfLaMurana, // 52 ラ・ムラーナの秘宝
  sacredOrb, // 53 生命の宝珠
  map, // 54 地図
  originSeal, // 55 始まりの印
  birthSeal, // 56 誕生の印
  lifeSeal, // 57 営みの印
  deathSeal, // 58 滅びの印
  sweetClothing, // 59 あぶねぇ水着
}

export type RomNumber = romNumbers;
export enum romNumbers {
  gameMaster, // 0 ⑩倍ｶｰﾄﾘｯｼ
  gameMaster2, // 1 新⑩倍ｶｰﾄﾘｯｼ
  glyphReader, // 2 古文書ﾘｰﾀﾞｰ
  ruinsRam8k, // 3 いせきRAM8K
  ruinsRam16k, // 4 いせきRAM16K
  unreleasedRom, // 5 かいはつちゅうしROM
  pr3, // 6 PR3
  gr3, // 7 GR3
  athleticLand, // 8 わんはﾟくｱｽﾚﾁｯｸ
  antarcticAdventure, // 9 けっきょくﾅﾝｷｮｸたﾞいほﾞうけん
  monkeyAcademy, // 10 もんﾀくんのいち･に･さんすう
  timePilot, // 11 ﾀｲﾑﾊﾟｲﾛｯﾄ
  frogger, // 12 ﾌﾛｯｶﾞｰ
  superCobra, // 13 ｽｰﾊﾟｰｺﾌﾞﾗ
  videoHustler, // 14 ﾋﾞﾃﾞｵﾊｽﾗｰ
  mahjongDojo, // 15 ｺﾅﾐのまぁしﾞゃんとﾞうしﾞょう
  hyperOlympic1, // 16 ﾊｲﾊﾟｰｵﾘﾝﾋﾟｯｸ1
  hyperOlympic2, // 17 ﾊｲﾊﾟｰｵﾘﾝﾋﾟｯｸ2
  hyperOlympic3, // 18 ﾊｲﾊﾟｰｵﾘﾝﾋﾟｯｸ3
  circusCharlier, // 19 ｻｰｶｽﾁｬｰﾘｰ
  magicalTree, // 20 ﾏｼﾞｶﾙﾂﾘｰ
  comicBakery, // 21 ほﾟんほﾟこﾊﾟﾝ
  hyperSports1, // 22 ﾊｲﾊﾟｰｽﾎﾟｰﾂ1
  hyperSports2, // 23 ﾊｲﾊﾟｰｽﾎﾟｰﾂ2
  hyperSports3, // 24 ﾊｲﾊﾟｰｽﾎﾟｰﾂ3
  cabbagePatchKids, // 25 ｷｬﾍﾞｯｼﾞﾊﾟｯﾁｷｯｽﾞ
  hyperRally, // 26 ﾊｲﾊﾟｰﾗﾘｰ
  konamiTennis, // 27 ｺﾅﾐのﾃﾆｽ
  skyJaguar, // 28 ｽｶｲｼﾞｬｶﾞｰ
  konamiPinball, // 29 ｺﾅﾐのﾋﾟﾝﾎﾞｰﾙ
  konamiGolf, // 30 ｺﾅﾐのｺﾞﾙﾌ
  konamiBaseball, // 31 ｺﾅﾐのﾍﾞｰｽﾎﾞｰﾙ
  yieArKungFu, // 32 ｲｰｱﾙｶﾝﾌｰ
  kingsValley, // 33 ｵｳｹのたに
  mopiRanger, // 34 ﾓﾋﾟﾚﾝｼﾞｬｰ
  pippols, // 35 ﾋﾟﾎﾟﾙｽ
  roadFighter, // 36 ﾛｰﾄﾞﾌｧｲﾀｰ
  konamiPingPong, // 37 ｺﾅﾐのﾋﾟﾝﾎﾟﾝ
  konamiSoccer, // 38 ｺﾅﾐのｻｯｶｰ
  goonies, // 39 ｸﾞｰﾆｰｽﾞ
  konamiBoxing, // 40 ｺﾅﾐのﾎﾞｸｼﾝｸﾞ
  yieArKungFu2, // 41 ｲｰｶﾞｰｺｳﾃｲのきﾞゃくしゅう
  knightmare, // 42 #しﾞょうてﾞんせつ
  twinbee, // 43 ﾂｲﾝﾋﾞｰ
  shinSynthesizer, // 44 新}ｻｲｻﾞｰ
  gradius, // 45 ｸﾞﾗﾃﾞｨｳｽ
  penguinAdventure, // 46 ﾕﾒたいりくｱﾄﾞﾍﾞﾝﾁｬｰ
  castlevania, // 47 ｱｸﾏしﾞょうﾄﾞﾗｷｭﾗ
  kingKong2, // 48 ｷﾝｸﾞｺﾝｸﾞ2ｰよみかﾞえるてﾞんせつｰ
  qbert, // 49 Qﾊﾞｰﾄ
  firebird, // 50 ﾋのとり
  ganbareGoemon, // 51 かﾞんはﾞれｺﾞｴﾓﾝからくりとﾞうちゅう
  mazeOfGalious, // 52 ｶﾞﾘｳｽのめいきゅう
  metalGear, // 53 ﾒﾀﾙｷﾞｱ
  gradius2, // 54 ｸﾞﾗﾃﾞｨｳｽ2
  f1Spirit, // 55 F1ｽﾋﾟﾘｯﾄ
  usas, // 56 ｳｼｬｽ
  shalom, // 57 ｼｬﾛﾑ
  breakShot, // 58 ﾌﾞﾚｲｸｼｮｯﾄ
  pennantRace, // 59 けﾞきとつﾍﾟﾅﾝﾄﾚｰｽ
  salamander, // 60 ｻﾗﾏﾝﾀﾞ
  parodius, // 61 ﾊﾟﾛﾃﾞｨｳｽ
  sealOfElGiza, // 62 ｴﾙｷﾞｰｻﾞのﾌｳ;
  contra, // 63 ｺﾝﾄﾗ
  heavenAndEarth, // 64 天と地と
  nemesis3, // 65 ｺﾞｰﾌｧｰのやほﾞうｴﾋﾟｿｰﾄﾞII
  mahjongWizard, // 66 ﾊｲの#しﾞゅつし
  pennantRace2, // 67 けﾞきとつﾍﾟﾅﾝﾄﾚｰｽ2
  metalGear2, // 68 ﾒﾀﾙｷﾞｱ2 ｿﾘｯﾄﾞｽﾈｰｸ
  spaceManbow, // 69 ｽﾍﾟｰｽﾏﾝﾎﾞｳ
  quarth, // 70 ｸｫｰｽ
  kingsValleyDisk, // 71 ｵｳｹのたにDISKはﾞん
  divinerSensation, // 72 ｺﾅﾐのうらないｾﾝｾｰｼｮﾝ
  snatcher, // 73 ｽﾅｯﾁｬｰ
  f1Spirit3d, // 74 F1ｽﾋﾟﾘｯﾄ3Dｽﾍﾟｼｬﾙ
  gameCollection1, // 75 ｺﾅﾐｹﾞｰﾑｺﾚｸｼｮﾝ1
  gameCollection2, // 76 ｺﾅﾐｹﾞｰﾑｺﾚｸｼｮﾝ2
  gameCollection3, // 77 ｺﾅﾐｹﾞｰﾑｺﾚｸｼｮﾝ3
  gameCollection4, // 78 ｺﾅﾐｹﾞｰﾑｺﾚｸｼｮﾝ4
  gameCollectionEX, // 79 ｺﾅﾐｹﾞｰﾑｺﾚｸｼｮﾝはﾞんかﾞいへん
  sdSnatcher, // 80 SDｽﾅｯﾁｬｰ
  badlands, // 81 ﾊﾞｯﾄﾞﾗﾝｽﾞ
  gradius2Beta, // 82 ｸﾞﾗﾃﾞｨｳｽ2ﾍﾞｰﾀ
  a1Spirit, // 83 A1ｽﾋﾟﾘｯﾄ
}
