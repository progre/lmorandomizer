import assert from '../../../assert';
import Item from './Item';
import Spot from './Spot';

export default class Storage {
  allItems: ReadonlyArray<Item>;

  static fromObject(obj: Storage) {
    const thiz = new this(
      obj.allRequirementNames,
      obj.mainWeaponShutters.map(x => ({
        spot: x.spot,
        item: new Item(x.item.name, x.item.type, x.item.number, x.item.count, x.item.flag),
      })),
      obj.subWeaponShutters.map(x => ({
        spot: x.spot,
        item: new Item(x.item.name, x.item.type, x.item.number, x.item.count, x.item.flag),
      })),
      obj.chests.map(x => ({
        spot: x.spot,
        item: new Item(x.item.name, x.item.type, x.item.number, x.item.count, x.item.flag),
      })),
      obj.sealChests.map(x => ({
        spot: x.spot,
        item: new Item(x.item.name, x.item.type, x.item.number, x.item.count, x.item.flag),
      })),
      obj.shops.map(x => ({
        spot: x.spot,
        items: <[Item, Item, Item]>x.items.map(y =>
          new Item(y.name, y.type, y.number, y.count, y.flag),
        ),
      })),
    );
    thiz.allItems = obj.allItems.map(x => new Item(x.name, x.type, x.number, x.count, x.flag));
    return thiz;
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
}
