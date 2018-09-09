import assert from 'assert';
import ScriptDat from '../../util/scriptdat/ScriptDat';
import { ShopItemData } from '../../util/scriptdat/ShopItemsData';
import Item from './Item';
import Spot from './Spot';
import Supplements from './Supplements';
import Storage from './Storage';

export default function createSource(scriptDat: ScriptDat, supplements: Supplements) {
  const allItems = getAllItems(scriptDat, supplements);
  const enumerateItems = [
    ...allItems.chests,
    ...allItems.shops.reduce<Item[]>((p, c) => [...p, ...c], []),
  ];
  warnMissingRequirements(supplements, enumerateItems);
  const nightSurfaceCount = 3;
  const chestDataList = scriptDat.chests();
  assert.equal(chestDataList.length, supplements.chests.length + nightSurfaceCount);
  const shops = scriptDat.shops();
  return new Storage(
    supplements.chests.map((supplement, i) => {
      const datum = chestDataList[i];
      const spot = new Spot(
        'chest',
        parseRequirements(supplement.requirements || null, enumerateItems),
        null,
      );
      return { spot, item: createItemFromChest(supplement.name, datum) };
    }),
    <{ spot: Spot; items: [Item, Item, Item] }[]>allItems.shops.map((items, i) => {
      const supplement = supplements.shops[i];
      const shop = shops[i];
      const spot = new Spot(
        'shop',
        parseRequirements(supplement.requirements || null, enumerateItems),
        shop.talkNumber,
      );
      return {
        spot,
        items,
      };
    }),
  );
}

function getAllItems(scriptDat: ScriptDat, supplements: Supplements) {
  const nightSurfaceCount = 3;
  const wareNoMiseCount = 1;
  const chestDataList = scriptDat.chests();
  assert.equal(chestDataList.length, supplements.chests.length + nightSurfaceCount);
  const shopDataList = scriptDat.shops();
  assert.equal(shopDataList.length, supplements.shops.length + wareNoMiseCount);
  return {
    chests: supplements.chests.map((supplement, i) => {
      const datum = chestDataList[i];
      return createItemFromChest(supplement.name, datum);
    }),
    shops: supplements.shops.map<[Item, Item, Item]>((supplement, i) => {
      const shop = shopDataList[i];
      const names = supplement.names.split(',').map(x => x.trim());
      assert.equal(names.length, 3);
      return [
        createItemFromShop(names[0], shop.items[0]),
        createItemFromShop(names[1], shop.items[1]),
        createItemFromShop(names[2], shop.items[2]),
      ];
    }),
  };
}

function createItemFromChest(
  name: string,
  data: { chestItemNumber: number; flag: number },
) {
  return new Item(
    name,
    data.chestItemNumber >= 100 ? 'rom' : 'equipment',
    (
      data.chestItemNumber >= 100 ? data.chestItemNumber - 100 : data.chestItemNumber
    ),
    1, // Count of chest item is always 1.
    data.flag,
  );
}

function createItemFromShop(name: string, data: ShopItemData) {
  return new Item(
    name,
    data.type === 0 ? 'subWeapon'
      : data.type === 1 ? 'equipment'
        : data.type === 2 ? 'rom'
          : (() => { throw new Error(); })(),
    data.number,
    data.count,
    data.flag,
  );
}

function parseRequirements(
  requirements: ReadonlyArray<ReadonlyArray<string>> | null,
  allItems: ReadonlyArray<Item>,
) {
  if (requirements == null) {
    return null;
  }
  const missings = [...new Set(
    requirements
      .reduce((p, c) => [...p, ...c], [])
      .filter(x => allItems.every(y => y.name !== x))
  )].sort();
  missings.forEach((missing) => {
    // console.warn(`WARN: missing item: ${missing}`);
  });
  return requirements.map(y => (
    y.map(z => allItems.filter(w => w.name === z)[0])
      .filter(z => z != null)
  ));
}

function warnMissingRequirements(
  supplements: Supplements,
  allItems: ReadonlyArray<Item>,
) {
  [...new Set(
    [
      supplements.mainWeapons.map(x => x.requirements || []),
      supplements.subWeapons.map(x => x.requirements || []),
      supplements.chests.map(x => x.requirements || []),
      supplements.shops.map(x => x.requirements || []),
    ]
      .reduce((p, c) => [...p, ...c], [])
      .reduce((p, c) => [...p, ...c], [])
      .reduce((p, c) => [...p, ...c], [])
  )]
    .filter(x => allItems.every(y => y.name !== x))
    .sort()
    .forEach(x => {
      console.warn(`WARN: missing item: ${x}`);
    });
}
