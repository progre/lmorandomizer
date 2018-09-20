import assert from 'assert';
import Item from './Item';
import Spot from './Spot';

export default class Storage {
  readonly allItems: ReadonlyArray<Item>;
  readonly allRequirementNames: ReadonlyArray<string>;

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

    this.allItems = (
      this.mainWeaponShutters.map(x => x.item)
        .concat(this.subWeaponShutters.map(x => x.item))
        .concat(this.chests.map(x => x.item))
        .concat(this.sealChests.map(x => x.item))
        .concat(this.shops.map(x => x.items).reduce((p, c) => p.concat(c), <Item[]>[]))
    );

    this.allRequirementNames = (() => {
      const set = new Set<string>();
      addSpotRequirementItemNamesTo(set, this.mainWeaponShutters);
      addSpotRequirementItemNamesTo(set, this.subWeaponShutters);
      addSpotRequirementItemNamesTo(set, this.chests);
      addSpotRequirementItemNamesTo(set, this.sealChests);
      addSpotRequirementItemNamesTo(set, this.shops);
      return [...set].sort();
    })();
  }

  reachableItemNames(
    currentItemNames: ReadonlyArray<string>,
    sacredOrbCount: number,
  ) {
    return (
      this.mainWeaponShutters
        .filter(x => x.spot.isReachable(currentItemNames, sacredOrbCount))
        .map(x => x.item.name)
        .concat((
          this.subWeaponShutters
            .filter(x => x.spot.isReachable(currentItemNames, sacredOrbCount))
            .map(x => x.item.name)
        ))
        .concat((
          this.chests
            .filter(x => x.spot.isReachable(currentItemNames, sacredOrbCount))
            .map(x => x.item.name)
        ))
        .concat((
          this.sealChests
            .filter(x => x.spot.isReachable(currentItemNames, sacredOrbCount))
            .map(x => x.item.name)
        ))
        .concat((
          this.shops
            .filter(x => x.spot.isReachable(currentItemNames, sacredOrbCount))
            .map(x => x.items)
            .reduce<ReadonlyArray<Item>>((p, c) => p.concat(c), [])
            .map(x => x.name)
        ))
    );
  }

  unreachables(
    currentItemNames: ReadonlyArray<string>,
    sacredOrbCount: number,
  ) {
    return new Storage(
      this.mainWeaponShutters
        .filter(x => !x.spot.isReachable(currentItemNames, sacredOrbCount)),
      this.subWeaponShutters
        .filter(x => !x.spot.isReachable(currentItemNames, sacredOrbCount)),
      this.chests
        .filter(x => !x.spot.isReachable(currentItemNames, sacredOrbCount)),
      this.sealChests
        .filter(x => !x.spot.isReachable(currentItemNames, sacredOrbCount)),
      this.shops
        .filter(x => !x.spot.isReachable(currentItemNames, sacredOrbCount)),
    );
  }
}

function addSpotRequirementItemNamesTo(
  set: Set<string>,
  items: ReadonlyArray<{ spot: Spot }>,
) {
  for (const item of items) {
    for (const group of item.spot.requirementItems || []) {
      for (const i of group) {
        set.add(i.name);
      }
    }
  }
}
