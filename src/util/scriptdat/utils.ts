import assert from 'assert';
import Item from '../../model/dataset/Item';
import Spot from '../../model/dataset/Spot';
import Storage from '../../model/dataset/Storage';
import { equipmentNumbers, subWeaponNumbers } from '../../model/randomizer/items';
import ShopItemsData, { ShopItemData } from './ShopItemsData';

export function replaceMainWeapon(txt: string, mainWeaponShutters: ReadonlyArray<Item>) {
  let idx = 0;
  return txt.split('\n').map((line) => {
    if (idx >= mainWeaponShutters.length) {
      return line;
    }
    if (!line.startsWith('<OBJECT 77,')) {
      return line;
    }
    const [x, y] = line.slice('<OBJECT 77,'.length, line.length - 1).split(',');
    const item = mainWeaponShutters[idx];
    idx += 1;
    return toObjectForShutter(Number(x), Number(y), item);
  }).join('\n');
}

export function replaceSubWeapon(txt: string, subWeaponShutters: ReadonlyArray<Item>) {
  let idx = 0;
  return txt.split('\n').map((line) => {
    if (idx >= subWeaponShutters.length) {
      return line;
    }
    if (!line.startsWith('<OBJECT 13,')) {
      return line;
    }
    const [x, y] = line.slice('<OBJECT 13,'.length, line.length - 1).split(',');
    const item = subWeaponShutters[idx];
    idx += 1;
    return toObjectForShutter(Number(x), Number(y), item);
  }).join('\n');
}

function toObjectForShutter(x: number, y: number, item: Item) {
  switch (item.type) {
    case 'mainWeapon':
      return `<OBJECT 77,${x},${y},${item.number},${item.flag},-1,-1>`;
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return `<OBJECT 13,${x},${y},${item.number},${item.count},${item.flag},-1>`;
    case 'equipment':
      return `<OBJECT 1,${x},${y},40,${item.number},${item.flag},-1>`;
    case 'rom':
      return `<OBJECT 1,${x},${y},40,${100 + item.number},${item.flag},-1>`;
    default: throw new Error();
  }
}

export function replaceChests(txt: string, shuffled: Storage) {
  let idx = 0;
  return txt.split('\n').map((line) => {
    if (idx >= shuffled.chests.length) {
      return line;
    }
    if (!line.startsWith('<OBJECT 1,')) {
      return line;
    }
    const params = line.slice('<OBJECT 1,'.length, line.length - 1).split(',');
    // アンクジュエル、印、手前より開けは無視
    if (
      params[3] === '-1'
      || params[3] === String(equipmentNumbers.twinStatue)
      || params[3] === String(equipmentNumbers.sweetClothing)
    ) {
      return line;
    }
    assert.equal(shuffled.chests[idx].spot.type, 'chest');
    const item = shuffled.chests[idx].item;
    idx += 1;
    const [x, y, open] = line.slice('<OBJECT 1,'.length, line.length - 1).split(',');
    return toObjectForChest(Number(x), Number(y), Number(open), item);
  }).join('\n');
}

function toObjectForChest(x: number, y: number, open: number, item: Item) {
  switch (item.type) {
    case 'mainWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return `<OBJECT 77,${x},${y},${item.number},${item.flag},-1,-1>`
        + '<START 99999,1>'
        + `<START ${open},1>`
        + `<START ${item.flag},0>`
        + `</OBJECT>`
        + `<OBJECT 1,${x},${y},${open},-1,${open},-1>`;
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return `<OBJECT 13,${x},${y},${item.number},${item.count},${item.flag},-1>`
        + '<START 99999,1>'
        + `<START ${open},1>`
        + `<START ${item.flag},0>`
        + `</OBJECT>`
        + `<OBJECT 1,${x},${y},${open},-1,${open},-1>`;
    case 'equipment':
      return `<OBJECT 1,${x},${y},${open},${item.number},${item.flag},-1>`;
    case 'rom':
      return `<OBJECT 1,${x},${y},${open},${100 + item.number},${item.flag},-1>`;
    default: throw new Error();
  }
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
  stringItemType: 'mainWeapon' | 'subWeapon' | 'equipment' | 'rom',
) {
  switch (stringItemType) {
    case 'mainWeapon': throw new Error();
    case 'subWeapon': return 0;
    case 'equipment': return 1;
    case 'rom': return 2;
    default: throw new Error();
  }
}
