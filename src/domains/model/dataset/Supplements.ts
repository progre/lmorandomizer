export type Requirement = string;

export default interface Supplements {
  readonly mainWeapons: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  readonly subWeapons: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  readonly chests: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  readonly seals: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  readonly shops: ReadonlyArray<{
    names: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
}
