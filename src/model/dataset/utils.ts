import assert from 'assert';
import ScriptDat from '../../util/scriptdat/ScriptDat';
import { ShopItemData } from '../../util/scriptdat/ShopItemsData';
import Spot from './Spot';
import { Item, Storage, Supplements } from './types';

export function getAllRequirements(supplements: Supplements) {
  return [...new Set([
    ...getAllRequirementsFromItems(supplements.mainWeapons),
    ...getAllRequirementsFromItems(supplements.subWeapons),
    ...getAllRequirementsFromItems(supplements.chests),
    ...getAllRequirementsFromItems(supplements.shops),
  ])].sort();
}

function getAllRequirementsFromItems(
  items: ReadonlyArray<{ requirements?: ReadonlyArray<ReadonlyArray<string>> }>,
) {
  return items
    .filter(x => x.requirements != null)
    .map(x => x.requirements!.reduce((p, c) => [...p, ...c], []))
    .reduce((p, c) => [...p, ...c], []);
}

export function getAllItemNames(supplements: Supplements) {
  return [
    ...supplements.mainWeapons.map(x => x.name),
    ...supplements.subWeapons.map(x => x.name),
    ...supplements.chests.map(x => x.name),
    ...supplements.shops
      .map(x => x.names.split(',').map(y => y.trim()))
      .reduce((p, c) => [...p, ...c], []),
  ];
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

export function getSource(scriptDat: ScriptDat, supplements: Supplements): Storage {
  const allItems = getAllItems(scriptDat, supplements);
  const enumerateItems = [
    ...allItems.chests,
    ...allItems.shops.reduce<Item[]>((p, c) => [...p, ...c], []),
  ];
  const nightSurfaceCount = 3;
  const chestDataList = scriptDat.chests();
  assert.equal(chestDataList.length, supplements.chests.length + nightSurfaceCount);
  const shops = scriptDat.shops();
  return {
    chests: supplements.chests.map((supplement, i) => {
      const datum = chestDataList[i];
      const spot = new Spot(
        'chest',
        parseRequirements(supplement.requirements, enumerateItems),
        null,
      );
      return { spot, item: createItemFromChest(supplement.name, datum) };
    }),
    shops: <{ spot: Spot; items: [Item, Item, Item] }[]>allItems.shops.map((items, i) => {
      const supplement = supplements.shops[i];
      const shop = shops[i];
      const spot = new Spot(
        'shop',
        parseRequirements(supplement.requirements, enumerateItems),
        shop.talkNumber,
      );
      return {
        spot,
        items,
      };
    }),
  };
}

function createItemFromChest(
  name: string,
  data: { chestItemNumber: number; flag: number },
): Item {
  return {
    name,
    type: data.chestItemNumber >= 100 ? 'rom' : 'equipment',
    number: (
      data.chestItemNumber >= 100 ? data.chestItemNumber - 100 : data.chestItemNumber
    ),
    count: 1, // Count of chest item is always 1.
    flag: data.flag,
  };
}

function createItemFromShop(name: string, data: ShopItemData): Item {
  return {
    name,
    type: data.type === 0 ? 'subWeapon'
      : data.type === 1 ? 'equipment'
        : data.type === 2 ? 'rom'
          : (() => { throw new Error(); })(),
    number: data.number,
    count: data.count,
    flag: data.flag,
  };
}

function parseRequirements(
  requirements: ReadonlyArray<ReadonlyArray<string>> | undefined,
  allItems: ReadonlyArray<Item>,
) {
  return requirements == null
    ? null
    : requirements.map(y => (
      y.map(z => allItems.filter(w => w.name === z)[0])
        .filter(z => z != null)
    ));
}
