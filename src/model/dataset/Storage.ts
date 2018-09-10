import Item from './Item';
import Spot from './Spot';

export default class Storage {
  constructor(
    public mainWeaponShutters: ReadonlyArray<{ spot: Spot; item: Item }>,
    public subWeaponShutters: ReadonlyArray<{ spot: Spot; item: Item }>,
    public chests: ReadonlyArray<{ spot: Spot; item: Item }>,
    public shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>,
  ) {
  }

  allItems(): ReadonlyArray<Item> {
    return [
      ...this.mainWeaponShutters.map(x => x.item),
      ...this.subWeaponShutters.map(x => x.item),
      ...this.chests.map(x => x.item),
      ...this.shops.map(x => x.items).reduce((p, c) => [...p, ...c], <Item[]>[]),
    ];
  }

  allRequirements() {
    return [...new Set([
      ...getAllRequirementsFromItems(this.mainWeaponShutters.map(x => x.spot)),
      ...getAllRequirementsFromItems(this.subWeaponShutters.map(x => x.spot)),
      ...getAllRequirementsFromItems(this.chests.map(x => x.spot)),
      ...getAllRequirementsFromItems(this.shops.map(x => x.spot)),
    ])].sort();
  }

  reachableItems(currentItems: ReadonlyArray<Item>) {
    return [
      ...this.mainWeaponShutters
        .filter(x => x.spot.isReachable(currentItems))
        .map(x => x.item),
      ...this.subWeaponShutters
        .filter(x => x.spot.isReachable(currentItems))
        .map(x => x.item),
      ...this.chests
        .filter(x => x.spot.isReachable(currentItems))
        .map(x => x.item),
      ...this.shops
        .filter(x => x.spot.isReachable(currentItems))
        .map(x => x.items)
        .reduce<ReadonlyArray<Item>>((p, c) => [...p, ...c], []),
    ];
  }

  unreachables(currentItems: ReadonlyArray<Item>) {
    return new Storage(
      this.mainWeaponShutters.filter(x => !x.spot.isReachable(currentItems)),
      this.subWeaponShutters.filter(x => !x.spot.isReachable(currentItems)),
      this.chests.filter(x => !x.spot.isReachable(currentItems)),
      this.shops.filter(x => !x.spot.isReachable(currentItems)),
    );
  }
}

function getAllRequirementsFromItems(
  items: ReadonlyArray<{ requirementItems: ReadonlyArray<ReadonlyArray<Item>> | null }>,
) {
  return items
    .filter(x => x.requirementItems != null)
    .map(x => x.requirementItems!.reduce((p, c) => [...p, ...c], []))
    .reduce((p, c) => [...p, ...c], []);
}
