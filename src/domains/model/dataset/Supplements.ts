export type Requirement = string;

export default class Supplements {
  static readonly nightSurfaceSubWeaponCount = 1;
  static readonly nightSurfaceChestCount = 3;
  static readonly trueShrineOfTheMotherSealCount = 1;
  static readonly nightSurfacSealCount = 1;
  static readonly wareNoMiseCount = 1;
  public readonly mainWeapons: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  public readonly subWeapons: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  public readonly chests: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  public readonly seals: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  public readonly shops: ReadonlyArray<{
    names: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;

  static from_object(obj: Supplements) {
    return new this(
      obj.mainWeapons,
      obj.subWeapons,
      obj.chests,
      obj.seals,
      obj.shops,
    );
  }

  constructor(
    mainWeapons: ReadonlyArray<{
      name: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
    subWeapons: ReadonlyArray<{
      name: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
    chests: ReadonlyArray<{
      name: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
    seals: ReadonlyArray<{
      name: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
    shops: ReadonlyArray<{
      names: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
  ) {
    this.mainWeapons = mainWeapons;
    this.subWeapons = subWeapons;
    this.chests = chests;
    this.seals = seals;
    this.shops = shops;
  }
}
