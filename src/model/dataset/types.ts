import Spot from './Spot';

export type Requirement = string;

export interface Supplements {
  mainWeapons: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  subWeapons: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  chests: ReadonlyArray<{
    name: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
  // seals: ReadonlyArray<{
  //   name: string;
  //   requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  // }>;
  shops: ReadonlyArray<{
    names: string;
    requirements?: ReadonlyArray<ReadonlyArray<Requirement>>;
  }>;
}

export interface Storage {
  chests: ReadonlyArray<{ spot: Spot; item: Item }>;
  shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>;
}

export interface Item {
  name: string;
  type: 'subWeapon' | 'equipment' | 'rom';
  number: number;
  count: number;
  flag: number;
}
