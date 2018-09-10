export type Requirement = string;

export default class Supplements {
  static readonly nightSurfaceSubWeaponCount = 1;
  static readonly nightSurfaceChestCount = 3;
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
    // public seals: ReadonlyArray<{
    //   name: string;
    //   requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    // }>,
    public shops: ReadonlyArray<{
      names: string;
      requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
    }>,
  ) {
  }

  getAllRequirements() {
    return [...new Set([
      ...getAllRequirementsFromItems(this.mainWeapons),
      ...getAllRequirementsFromItems(this.subWeapons),
      ...getAllRequirementsFromItems(this.chests),
      ...getAllRequirementsFromItems(this.shops),
    ])].sort();
  }

  getAllItemNames() {
    return [
      ...this.mainWeapons.map(x => x.name),
      ...this.subWeapons.map(x => x.name),
      ...this.chests.map(x => x.name),
      ...this.shops
        .map(x => x.names.split(',').map(y => y.trim()))
        .reduce((p, c) => [...p, ...c], []),
    ];
  }
}

function getAllRequirementsFromItems(
  items: ReadonlyArray<{ requirements?: ReadonlyArray<ReadonlyArray<string>> }>,
) {
  return items
    .filter(x => x.requirements != null)
    .map(x => x.requirements!.reduce((p, c) => [...p, ...c], []))
    .reduce((p, c) => [...p, ...c], []);
}
