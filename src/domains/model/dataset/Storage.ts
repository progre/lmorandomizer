import assert from '../../../assert';
import Item from './Item';
import Spot from './Spot';

export default class Storage {
  readonly allItems: ReadonlyArray<Item>;

  static create(
    mainWeaponShutters: ReadonlyArray<{ spot: Spot; item: Item }>,
    subWeaponShutters: ReadonlyArray<{ spot: Spot; item: Item }>,
    chests: ReadonlyArray<{ spot: Spot; item: Item }>,
    sealChests: ReadonlyArray<{ spot: Spot; item: Item }>,
    shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>,
  ) {
    return new this(
      (() => {
        const set = new Set<string>();
        addSpotRequirementItemNamesTo(set, mainWeaponShutters);
        addSpotRequirementItemNamesTo(set, subWeaponShutters);
        addSpotRequirementItemNamesTo(set, chests);
        addSpotRequirementItemNamesTo(set, sealChests);
        addSpotRequirementItemNamesTo(set, shops);
        return [...set].sort();
      })(),
      mainWeaponShutters,
      subWeaponShutters,
      chests,
      sealChests,
      shops,
    );
  }

  constructor(
    public readonly allRequirementNames: ReadonlyArray<string>,
    public readonly mainWeaponShutters: ReadonlyArray<{ spot: Spot; item: Item }>,
    public readonly subWeaponShutters: ReadonlyArray<{ spot: Spot; item: Item }>,
    public readonly chests: ReadonlyArray<{ spot: Spot; item: Item }>,
    public readonly sealChests: ReadonlyArray<{ spot: Spot; item: Item }>,
    public readonly shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>,
  ) {
    assert(mainWeaponShutters.every(x => x.spot.type === 'weaponShutter'));
    assert(subWeaponShutters.every(x => x.spot.type === 'weaponShutter'));
    assert(chests.every(x => x.spot.type === 'chest'));
    assert(sealChests.every(x => x.spot.type === 'sealChest'));
    assert(shops.every(x => x.spot.type === 'shop'));

    const allItems = (
      this.mainWeaponShutters.map(x => x.item)
        .concat(this.subWeaponShutters.map(x => x.item))
        .concat(this.chests.map(x => x.item))
        .concat(this.sealChests.map(x => x.item))
        .concat(this.shops.map(x => x.items).reduce((p, c) => p.concat(c), <Item[]>[]))
    );
    allItems.sort((a, b) => (
      Number(a.canDisplayInShop()) - Number(b.canDisplayInShop())
    ));
    this.allItems = allItems;
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
      this.allRequirementNames,
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
