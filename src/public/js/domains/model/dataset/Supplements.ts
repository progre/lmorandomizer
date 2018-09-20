import assert from 'assert';

export type Requirement = string;

export default class Supplements {
  static readonly nightSurfaceSubWeaponCount = 1;
  static readonly nightSurfaceChestCount = 3;
  static readonly trueShrineOfTheMotherSealCount = 1;
  static readonly nightSurfacSealCount = 1;
  static readonly wareNoMiseCount = 1;

  constructor(
    public mainWeapons: ReadonlyArray<{
      name: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
    public subWeapons: ReadonlyArray<{
      name: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
    public chests: ReadonlyArray<{
      name: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
    public seals: ReadonlyArray<{
      name: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
    public shops: ReadonlyArray<{
      names: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
  ) {
    assert.deepEqual(
      chests.find(x => x.name === 'iceCape')!.requirements,
      [
        ['ankhJewel:templeOfTheSun', 'bronzeMirror', 'shuriken', 'shurikenAmmo'],
        ['holyGrail', 'flareGun', 'grappleClaw'],
        // tslint:disable-next-line:max-line-length
        // ['anchor', 'knife', 'bronzeMirror', 'ankhJewel:gateOfGuidance', 'flareGun', 'grappleClaw'],
        ['bronzeMirror', 'ankhJewel:mausoleumOfTheGiants', 'flareGun', 'grappleClaw'],
        ['holyGrail', 'flareGun', 'feather'],
        // ['anchor', 'knife', 'bronzeMirror', 'ankhJewel:gateOfGuidance', 'flareGun', 'feather'],
        ['bronzeMirror', 'ankhJewel:mausoleumOfTheGiants', 'flareGun', 'feather'],
      ],
    );
  }
}
