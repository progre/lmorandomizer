import { PhysicalConditionGroups } from './conditions';
import { equipmentNumbers, subWeaponNumbers } from './items';
import { Chest } from './places';

export const CHESTS: ReadonlyArray<Chest> = (() => {
  // tslint:disable-next-line:max-func-body-length
  const list: ReadonlyArray<[string, PhysicalConditionGroups]> = [
    // 導きの門
    ['<OBJECT 1,32768,30720,60,47,794,-1>', []], // C-1(財宝)
    ['<OBJECT 1,30720,6144,41,53,717,-1>', []], // D-1(生命の宝珠)
    [ // B-5(十字架)
      '<OBJECT 1,49152,38912,42,22,774,-1>',
      [[
        { type: 'equipment', payload: equipmentNumbers.birthSeal },
        { type: 'subWeapon', payload: subWeaponNumbers.flareGun },
      ]],
    ],
    ['<OBJECT 1,47104,22528,43,54,727,-1>', []], // C-3(地図)
    [ // A-3(新10倍カートリッジ)
      '<OBJECT 1,2048,30720,44,101,801,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachSavePointOfGateOfIllusion' },
      ]],
    ],
    ['<OBJECT 1,45056,14336,62,5,755,-1>', []], // B-4(聖杯)

    // 地上
    [ // A-3(羽)
      '<OBJECT 1,2048,22528,107,17,769,-1>',
      [[
        { type: 'equipment', payload: equipmentNumbers.serpentStaff },
        // TODO: anyWeaponEnhancement
      ]],
    ],
    ['<OBJECT 1,2048,14336,102,1,753,-1>', []], // I-2(ほら貝)
    // '<OBJECT 1,53248,22528,1039,-1,1039,-1>', null, 印
    [ // L-3(生命の宝珠)
      '<OBJECT 1,45056,30720,106,53,725,-1>',
      [[
        { type: 'equipment', payload: equipmentNumbers.scalesphere },
      ]], // TODO: たくさん宝珠でもいけるかも
    ],
    // 巨人霊廟
    ['<OBJECT 1,36864,6144,142,54,728,-1>', []], // C-1(地図)
    // '<OBJECT 1,57344,14336,143,-1,744,-1>', null, アンクジュエル
    ['<OBJECT 1,24576,14336,144,53,718,-1>', []], // F-4(生命の宝珠)

    // 太陽神殿
    ['<OBJECT 1,22528,30720,185,15,767,-1>', []], // E-6(銅鏡)
    ['<OBJECT 1,59392,30720,180,54,729,-1>', []], // C-1(地図)
    [ // F-3(お守り)
      '<OBJECT 1,57344,30720,222,8,758,-1>',
      []// TODO: [[{ type: 'event', payload: 'beatViy' }]],
    ],
    [ // G-4(ブーツ)
      '<OBJECT 1,43008,22528,182,16,768,-1>',
      [
        [{ type: 'equipment', payload: equipmentNumbers.feather }],
        // TODO: [{ type: 'event', payload: 'runWaterFromSpringToTemple' }],
      ],
    ],
    ['<OBJECT 1,36864,30720,183,53,719,-1>', []], // D-4(生命の宝珠)
    // '<OBJECT 1,2048,6144,184,-1,745,-1>', null, 太陽神殿アンクジュエル

    // 空の水源
    [ // A-4(生命の宝珠)
      '<OBJECT 1,30720,6144,241,53,720,-1>',
      [[{ type: 'equipment', payload: equipmentNumbers.lifeSeal }]],
    ],
    [ // C-5(グローブ) 玉鱗または複数個の生命の宝珠
      '<OBJECT 1,14336,6144,243,12,764,-1>',
      [] // TODO: [[{ type: 'event', payload: 'runWaterInSpring' }]],
    ],
    [ // B-6(地図)
      '<OBJECT 1,30720,6144,242,54,730,-1>',
      [[{ type: 'equipment', payload: equipmentNumbers.helmet }]],
    ],
    [ // D-1(玉鱗) 始まりの印
      '<OBJECT 1,2048,14336,245,33,785,-1>',
      [[
        { type: 'equipment', payload: equipmentNumbers.helmet },
        { type: 'equipment', payload: equipmentNumbers.originSeal },
      ]],
    ],
    // '<OBJECT 1,4096,14336,244,-1,795,-1>', null, 印
    // '<OBJECT 1,59392,6144,246,-1,746,-1>', null, アンクジュエル

    // 灼熱洞窟
    ['<OBJECT 1,2048,14336,283,54,731,-1>', []], // E-1(地図)
    [ // G-5(かぎ爪)
      '<OBJECT 1,59392,6144,281,20,772,-1>',
      [[{ type: 'equipment', payload: equipmentNumbers.sacredOrb }]], // TODO: たりないかも
    ],

    // 死滅の間
    ['<OBJECT 1,2048,6144,320,53,722,-1>', []], // C-3(生命の宝珠) 但し暗闇を通過する必要あり
    ['<OBJECT 1,43008,22528,321,54,732,-1>', []], // C-4(地図) 但し暗闇を通過する必要あり
    // '<OBJECT 1,59392,22528,322,-1,797,-1>', null, 営みの印

    // 無限回廊
    ['<OBJECT 1,2048,22528,400,54,733,-1>', []], // A-1(地図)
    // '<OBJECT 1,8192,6144,419,14,766,-1>', null, // A-3(双子の像)
    // '<OBJECT 1,8192,6144,420,14,766,0>', null, // A-3(双子の像)

    // 聖母の祠
    [ // B-2(水晶のドクロ)
      '<OBJECT 1,6144,30720,453,34,786,-1>',
      [[
        // TODO: { type: 'event', payload: 'beatEndlessCorridor' },
        { type: 'equipment', payload: equipmentNumbers.lifeSeal },
        { type: 'equipment', payload: equipmentNumbers.mulanaTalisman },
      ]],
    ],
    [ // C-2(生命の宝珠)
      '<OBJECT 1,30720,38912,452,53,724,-1>',
      [[
        // TODO: { type: 'event', payload: 'beatEndlessCorridor' },
        { type: 'equipment', payload: equipmentNumbers.originSeal },
        { type: 'equipment', payload: equipmentNumbers.birthSeal },
        { type: 'equipment', payload: equipmentNumbers.lifeSeal },
        { type: 'equipment', payload: equipmentNumbers.deathSeal },
      ]],
    ],
    [ // E-2(日記)
      '<OBJECT 1,59392,6144,451,41,759,-1>',
      [[
        // TODO: { type: 'event', payload: 'beatEndlessCorridor' },
        { type: 'equipment', payload: equipmentNumbers.talisman },
        { type: 'equipment', payload: equipmentNumbers.lifeSeal },
        { type: 'equipment', payload: equipmentNumbers.lampOfTime },
      ]],
    ],
    [ // F-3(地図)
      '<OBJECT 1,30720,14336,454,54,734,-1>',
      [[
        // TODO: { type: 'event', payload: 'releaseTwins' },
        { type: 'equipment', payload: equipmentNumbers.dragonBone },
      ]],
    ],
    // '<OBJECT 1,30720,14336,455,-1,798,-1>', null, // A-3(滅びの印)

    // 双連迷宮
    [ // B-2(リング)
      '<OBJECT 1,30720,6144,481,11,763,-1>',
      []// TODO: [[{ type: 'event', payload: 'releaseTwins' }]],
    ],
    [ // D-2(地図)
      '<OBJECT 1,55296,14336,482,54,735,-1>',
      []// TODO: [[{ type: 'event', payload: 'releaseTwins' }]],
    ],
    [ // C-3(生命の宝珠)
      '<OBJECT 1,18432,28672,483,53,723,-1>',
      []// TODO: [[{ type: 'event', payload: 'releaseTwins' }]],
    ],

    // 双連迷宮（裏）
    // '<OBJECT 1,28672,10240,490,-1,749,-1>', null, 偽アンクジュエル

    // 迷いの門
    [ // C-3(無限の鍵)
      '<OBJECT 1,18432,30720,478,13,765,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachSavePointOfGateOfIllusion' },
        { type: 'equipment', payload: equipmentNumbers.miniDoll },
      ]],
    ],
    [ // A-4(防護服)
      '<OBJECT 1,2048,14336,522,7,757,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachFrontOfGateOfIllusion' },
        { type: 'equipment', payload: equipmentNumbers.anchor },
        // TODO: { type: 'mainWeapon', payload: mainWeapons.katana },
        // TODO: { type: 'rom', payload: 72 }, // 占いセンセーション
      ]],
    ],
    [ // D-5(地図)
      '<OBJECT 1,26624,14336,521,54,736,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachFrontOfGateOfIllusion' },
      ]],
    ],
    [ // E-1(はずみ車)
      '<OBJECT 1,32768,30720,523,37,789,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachBackDoorOfGateOfIllusion' },
        // TODO: { type: 'event', payload: 'reachFrontOfGateOfIllusion' },
        { type: 'equipment', payload: equipmentNumbers.lampOfTime },
      ]],
    ],

    // 巨人墓場
    [ // D-2(コントラ)
      '<OBJECT 1,43008,14336,40,163,863,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachBackdoorOfGraveyard' }]],
    ],
    [ // D-3(地図)
      '<OBJECT 1,38912,6144,542,54,737,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachFrontOfGraveyard' },
        { type: 'equipment', payload: equipmentNumbers.feather },
      ]],
    ],
    [ // D-4(ガントレット)
      '<OBJECT 1,14336,6144,543,10,762,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachFrontOfGraveyard' },
        { type: 'equipment', payload: equipmentNumbers.birthSeal },
        { type: 'equipment', payload: equipmentNumbers.grappleClaw },
      ]],
    ],

    // 女神の塔
    [ // B-7(A1スピリット)
      '<OBJECT 1,2048,38912,900,183,883,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachTowerOfTheGoddess' },
        { type: 'equipment', payload: equipmentNumbers.planeModel },
      ]],
    ],
    [ // D-2(飛行機模型)
      '<OBJECT 1,32768,28672,891,36,788,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachTowerOfTheGoddess' },
        { type: 'equipment', payload: equipmentNumbers.feather },
        { type: 'equipment', payload: equipmentNumbers.grappleClaw },
      ]],
    ],
    [ // C-8(地図)
      '<OBJECT 1,57344,38912,896,54,738,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachTowerOfTheGoddess' }]],
    ],
    [ // D-5(グラディウス)
      '<OBJECT 1,59392,38912,893,145,845,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachTowerOfTheGoddess' },
        { type: 'equipment', payload: equipmentNumbers.feather },
      ]],
    ],
    [ // A-5(真実の目)
      '<OBJECT 1,36864,22528,894,29,781,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachTowerOfTheGoddess' }]],
    ],

    // 月光聖殿
    [ // D-6(死者の書)
      '<OBJECT 1,38912,38912,564,23,775,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachFrontOfTempleOfMoonlight' },
        { type: 'equipment', payload: equipmentNumbers.birthSeal },
      ]],
    ],
    [ // A-1(氷のマント)
      '<OBJECT 1,30720,22528,560,31,783,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachSavePointOfTempleOfMoonlight' }]],
    ],
    [ // C-2(地図)
      '<OBJECT 1,59392,22528,561,54,739,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachSavePointOfTempleOfMoonlight' }]],
    ],
    // '<OBJECT 1,43008,22528,10,-1,-1,-1>', null, 手前より開け
    // '<OBJECT 1,47104,22528,11,-1,-1,-1>', null,
    // '<OBJECT 1,51200,22528,12,-1,-1,-1>', null,
    // '<OBJECT 1,55296,22528,13,-1,-1,-1>', null,
    [ // D-3(オカリナ)
      '<OBJECT 1,30720,30720,562,25,777,-1>',
      [[
        { type: 'subWeapon', payload: subWeaponNumbers.flareGun },
        { type: 'equipment', payload: equipmentNumbers.womanStatue },
      ]],
    ],
    [ // E-3(ヘビの杖)
      '<OBJECT 1,51200,6144,563,30,782,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachFrontOfTempleOfMoonlight' },
        { type: 'equipment', payload: equipmentNumbers.grappleClaw },
      ]],
    ],

    // 滅びの塔
    [ // E-1(くさび)
      '<OBJECT 1,28672,6144,593,35,787,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachBackDoorOfTowerOfRuin' }]],
    ],
    [ // B-3(地図)
      '<OBJECT 1,34816,14336,594,54,740,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachFrontOfTowerOfRuin' }]],
    ],
    // '<OBJECT 1,30720,30720,592,-1,747,-1>', null, アンクジュエル
    [ // G-5(生命の宝珠)
      '<OBJECT 1,34816,14336,591,53,721,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachFrontOfTowerOfRuin' }]],
    ],

    // 産声の間
    [ // F-2(器)
      '<OBJECT 1,57344,38912,619,39,791,-1>',
      [
        [
          // TODO: { type: 'event', payload: 'reachUpperOfChamberOfBirth' },
          { type: 'subWeapon', payload: subWeaponNumbers.silverShield },
        ],
        [
          // TODO: { type: 'event', payload: 'reachUpperOfChamberOfBirth' },
          { type: 'subWeapon', payload: subWeaponNumbers.angelShield },
        ],
      ],
    ],
    [ // G-2(香水)
      '<OBJECT 1,57344,14336,610,24,776,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachUpperOfChamberOfBirth' },
        { type: 'equipment', payload: equipmentNumbers.cogOfTheSoul },
      ]],
    ],
    [ // I-2(女性像)
      '<OBJECT 1,6144,6144,611,27,779,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachUpperOfChamberOfBirth' },
        { type: 'equipment', payload: equipmentNumbers.feather },
      ]],
    ],
    [ // D-5(地図)
      '<OBJECT 1,32768,38912,613,54,741,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachLowerOfChamberOfBirth' }]],
    ],
    [ // F-5(ポシェットキー)
      '<OBJECT 1,57344,14336,612,38,790,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachLowerOfChamberOfBirth' },
        { type: 'equipment', payload: equipmentNumbers.cogOfTheSoul },
      ]],
    ],

    // 次元回廊
    [ // D-3(まが玉)
      '<OBJECT 1,8192,38912,716,21,773,-1>',
      []// TODO: [[{ type: 'event', payload: 'beatTiamat' }]],
    ],
    [ // D-1(生命の宝珠)
      '<OBJECT 1,14336,24576,922,53,726,-1>',
      [[
        // TODO: { type: 'event', payload: 'reachDimensionalCorridor' }
        { type: 'equipment', payload: equipmentNumbers.feather },
        { type: 'equipment', payload: equipmentNumbers.lampOfTime },
      ]],
    ],
    // '<OBJECT 1,59392,6144,920,-1,750,-1>', null, アンクジュエル
    [
      '<OBJECT 1,2048,6144,920,54,742,-1>',
      []// TODO: [[{ type: 'event', payload: 'reachDimensionalCorridor' }]]
    ], // C-7(地図)

    // '<OBJECT 1,30720,28672,343,59,344,-1>', null, // あぶねぇ水着

    // 夜の地上
    // '<OBJECT 1,2048,22528,107,17,769,-1>', null,
    // '<OBJECT 1,2048,14336,102,1,753,-1>', null,
    // '<OBJECT 1,53248,22528,1039,-1,1039,-1>', null, 夜 誕生の印
    // '<OBJECT 1,45056,30720,106,53,725,-1>', null,
  ];
  return list.map(([source, conditionGroups]) => ({
    source,
    conditionGroups,
  }));
})();
