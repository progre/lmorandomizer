export type Item = MainWeaponItem | SubWeaponItem | EquipmentItem | ROMItem;
export interface MainWeaponItem { type: 'mainWeapon'; payload: MainWeapon; }
export interface SubWeaponItem { type: 'subWeapon'; payload: SubWeapon; }
export interface EquipmentItem { type: 'equipment'; payload: Equipment; }
export interface ROMItem { type: 'rom'; payload: ROM; }

export interface MainWeapon {
  num: MainWeaponNumber;
  flag: number;
}

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

export interface SubWeapon {
  num: SubWeaponNumber;
  flag: number;
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

export interface Equipment {
  num: EquipmentNumber;
  flag: number;
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

export interface ROM {
  num: number;
  flag: number;
}
