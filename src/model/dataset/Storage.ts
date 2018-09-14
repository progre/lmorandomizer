import assert from 'assert';
import Item from './Item';
import Spot from './Spot';

export default class Storage {
  constructor(
    public mainWeaponShutters: ReadonlyArray<{ spot: Spot; item: Item }>,
    public subWeaponShutters: ReadonlyArray<{ spot: Spot; item: Item }>,
    public chests: ReadonlyArray<{ spot: Spot; item: Item }>,
    public sealChests: ReadonlyArray<{ spot: Spot; item: Item }>,
    public shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>,
  ) {
    assert(mainWeaponShutters.every(x => x.spot.type === 'weaponShutter'));
    assert(subWeaponShutters.every(x => x.spot.type === 'weaponShutter'));
    assert(chests.every(x => x.spot.type === 'chest'));
    assert(sealChests.every(x => x.spot.type === 'sealChest'));
    assert(shops.every(x => x.spot.type === 'shop'));
  }

  allItems(): ReadonlyArray<Item> {
    return [
      ...this.mainWeaponShutters.map(x => x.item),
      ...this.subWeaponShutters.map(x => x.item),
      ...this.chests.map(x => x.item),
      ...this.sealChests.map(x => x.item),
      ...this.shops.map(x => x.items).reduce((p, c) => p.concat(c), <Item[]>[]),
    ];
  }

  allRequirements() {
    const set = new Set<Item>();
    [
      getAllRequirementsFromItems(this.mainWeaponShutters.map(x => x.spot)),
      getAllRequirementsFromItems(this.subWeaponShutters.map(x => x.spot)),
      getAllRequirementsFromItems(this.chests.map(x => x.spot)),
      getAllRequirementsFromItems(this.sealChests.map(x => x.spot)),
      getAllRequirementsFromItems(this.shops.map(x => x.spot)),
    ].forEach((x) => {
      x.forEach((y) => {
        set.add(y);
      });
    });
    return [...set].sort();
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
      ...this.sealChests
        .filter(x => x.spot.isReachable(currentItems))
        .map(x => x.item),
      ...this.shops
        .filter(x => x.spot.isReachable(currentItems))
        .map(x => x.items)
        .reduce<ReadonlyArray<Item>>((p, c) => p.concat(c), []),
    ];
  }

  unreachables(currentItems: ReadonlyArray<Item>) {
    return new Storage(
      this.mainWeaponShutters.filter(x => !x.spot.isReachable(currentItems)),
      this.subWeaponShutters.filter(x => !x.spot.isReachable(currentItems)),
      this.chests.filter(x => !x.spot.isReachable(currentItems)),
      this.sealChests.filter(x => !x.spot.isReachable(currentItems)),
      this.shops.filter(x => !x.spot.isReachable(currentItems)),
    );
  }
}

function getAllRequirementsFromItems(
  items: ReadonlyArray<{ requirementItems: ReadonlyArray<ReadonlyArray<Item>> | null }>,
) {
  return items
    .filter(x => x.requirementItems != null)
    .map(x => x.requirementItems!.reduce((p, c) => p.concat(c), []))
    .reduce((p, c) => p.concat(c), []);
}
