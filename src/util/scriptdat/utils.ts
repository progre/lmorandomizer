import assert from 'assert';
import Item from '../../model/dataset/Item';
import Spot from '../../model/dataset/Spot';
import Storage from '../../model/dataset/Storage';
import { equipmentNumbers } from '../../model/randomizer/items';
import ShopItemsData, { ShopItemData } from './ShopItemsData';
import { toTagsForChest, toTagsForSealChest, toTagsForShutter } from './tagsfactory';

export function replaceItems(txt: string, source: Storage) {
  let mainWeaponShutterItemsIdx = 0;
  let subWeaponShutterItemsIdx = 0;
  let chestItemsIdx = 0;
  let sealChestItemsIdx = 0;
  const lines = txt.split('\n');
  return lines.map((line, i) => {
    if (mainWeaponShutterItemsIdx < source.mainWeaponShutters.length) {
      const item = source.mainWeaponShutters[mainWeaponShutterItemsIdx].item;
      const newLine = replaceMainWeaponShutterItem(line, lines.slice(i + 1), item);
      if (newLine != null) {
        mainWeaponShutterItemsIdx += 1;
        return newLine;
      }
    }
    if (subWeaponShutterItemsIdx < source.subWeaponShutters.length) {
      const item = source.subWeaponShutters[subWeaponShutterItemsIdx].item;
      const newLine = replaceSubWeaponSpot(line, lines.slice(i + 1), item);
      if (newLine != null) {
        subWeaponShutterItemsIdx += 1;
        return newLine;
      }
    }
    if (chestItemsIdx < source.chests.length) {
      const item = source.chests[chestItemsIdx].item;
      const newLine = replaceChestItem(line, item);
      if (newLine != null) {
        chestItemsIdx += 1;
        return newLine;
      }
    }
    if (sealChestItemsIdx < source.sealChests.length) {
      const item = source.sealChests[sealChestItemsIdx].item;
      const newLine = replaceSealChestItem(line, item);
      if (newLine != null) {
        sealChestItemsIdx += 1;
        return newLine;
      }
    }
    return line;
  }).join('\n');
}

export function replaceMainWeaponShutterItem(
  line: string,
  nextLines: ReadonlyArray<string>,
  item: Item,
) {
  if (!line.startsWith('<OBJECT 77,')) {
    return null;
  }
  const checkFlag = getNextShutterCheckFlag(nextLines);
  const [x, y] = line.slice('<OBJECT 77,'.length, line.length - 1).split(',');
  return toTagsForShutter(Number(x), Number(y), checkFlag, item);
}

export function replaceSubWeaponSpot(
  line: string,
  nextLines: ReadonlyArray<string>,
  item: Item,
) {
  if (!line.startsWith('<OBJECT 13,')) {
    return null;
  }
  const [x, y, number] = line.slice('<OBJECT 13,'.length, line.length - 1).split(',');
  if (number === '7') {
    if (nextLines[0] === '<START 743,0>') {
      const wallCheckFlag = getNextWallCheckFlag(nextLines);
      return toTagsForShutter(Number(x), Number(y), wallCheckFlag, item);
    }
    return toTagsForSealChest(Number(x), Number(y), item);
  }
  const checkFlag = getNextShutterCheckFlag(nextLines);
  return toTagsForShutter(Number(x), Number(y), checkFlag, item);
}

function getNextShutterCheckFlag(lines: ReadonlyArray<string>) {
  for (const line of lines) {
    if (line === '</MAP>') {
      return 40;
    }
    if (!line.startsWith('<OBJECT 20,')) {
      continue;
    }
    const [, , checkFlag]
      = line.slice('<OBJECT 20,'.length, line.length - 1).split(',');
    return Number(checkFlag);
  }
  console.error(lines);
  throw new Error();
}

function getNextWallCheckFlag(lines: ReadonlyArray<string>) {
  for (const line of lines) {
    if (line === '</MAP>') {
      return 40;
    }
    if (!line.startsWith('<OBJECT 59,')) {
      continue;
    }
    const [, , , , checkFlag]
      = line.slice('<OBJECT 59,'.length, line.length - 1).split(',');
    return Number(checkFlag);
  }
  console.error(lines);
  throw new Error();
}

export function replaceChestItem(line: string, item: Item) {
  if (!line.startsWith('<OBJECT 1,')) {
    return null;
  }
  const params = line.slice('<OBJECT 1,'.length, line.length - 1).split(',');
  // アンクジュエル、印、手前より開けは無視
  if (
    params[3] === '-1'
    || params[3] === String(equipmentNumbers.twinStatue)
    || params[3] === String(equipmentNumbers.sweetClothing)
  ) {
    return null;
  }
  const [x, y, open] = line.slice('<OBJECT 1,'.length, line.length - 1).split(',');
  return toTagsForChest(Number(x), Number(y), Number(open), item);
}

export function replaceSealChestItem(line: string, item: Item) {
  if (!line.startsWith('<OBJECT 71,')) {
    return null;
  }
  const [x, y] = line.slice('<OBJECT 71,'.length, line.length - 1).split(',');
  return toTagsForSealChest(Number(x), Number(y), item);
}

export function replaceShops(
  txt: string,
  shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>,
) {
  const r = '<TALK>\n{}[^]+?{}</TALK>\n'.split('{}');
  return txt
    .split(new RegExp(`(${r[0]}${r[1]}${r[2]})`))
    .filter(x => x.length > 0)
    .map((x, i) => {
      const newShop = shops.filter(y => y.spot.talkNumber === i)[0];
      if (newShop == null) {
        return x;
      }
      const shopItemsDataRaw = x.split(new RegExp(`${r[0]}(${r[1]})${r[2]}`))[1];
      assert.notEqual(shopItemsDataRaw, null);
      const shopItemsData = ShopItemsData.parse(shopItemsDataRaw);
      const replaced = <[ShopItemData, ShopItemData, ShopItemData]>shopItemsData.map((item, j) => {
        const newShopItem = newShop.items[j];
        return {
          type: toIntegerItemType(newShopItem.type),
          number: newShopItem.number,
          price: item.price,
          count: newShopItem.count,
          flag: newShopItem.flag,
        };
      });
      return `<TALK>\n${ShopItemsData.stringify(replaced)}</TALK>\n`;
    })
    .join('');
}

function toIntegerItemType(
  stringItemType: 'mainWeapon' | 'subWeapon' | 'equipment' | 'rom' | 'seal',
) {
  switch (stringItemType) {
    case 'mainWeapon': throw new Error();
    case 'subWeapon': return 0;
    case 'equipment': return 1;
    case 'rom': return 2;
    case 'seal': throw new Error();
    default: throw new Error();
  }
}
