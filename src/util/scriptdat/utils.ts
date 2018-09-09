import assert from 'assert';
import Item from '../../model/dataset/Item';
import Spot from '../../model/dataset/Spot';
import Storage from '../../model/dataset/Storage';
import { equipmentNumbers, subWeaponNumbers } from '../../model/randomizer/items';
import ShopItemsData, { ShopItemData } from './ShopItemsData';

export function replaceChests(txt: string, shuffled: Storage) {
  let idx = 0;
  return txt.split('\n').map((x) => {
    if (idx >= shuffled.chests.length) {
      return x;
    }
    if (!x.startsWith('<OBJECT 1,')) {
      return x;
    }
    const params = x.slice('<OBJECT 1,'.length, x.length - 1).split(',');
    // アンクジュエル、印、手前より開けは無視
    if (
      params[3] === '-1'
      || params[3] === String(equipmentNumbers.twinStatue)
      || params[3] === String(equipmentNumbers.sweetClothing)
    ) {
      return x;
    }
    assert.equal(shuffled.chests[idx].spot.type, 'chest');
    const item = shuffled.chests[idx].item;
    const chest = replaceChest(x, item);
    idx += 1;
    return chest;
  }).join('\n');
}

function replaceChest(line: string, item: Item) {
  const params = line.slice('<OBJECT 1,'.length, line.length - 1).split(',');
  switch (item.type) {
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return `<OBJECT 13,${params[0]},${params[1]},${item.number},${item.count},${item.flag},-1>`
        + '<START 99999,1>'
        + `<START ${params[2]},1>`
        + `<START ${item.flag},0>`
        + `</OBJECT>`
        + `<OBJECT 1,${params[0]},${params[1]},${params[2]},-1,${params[2]},-1>`;
    case 'equipment': params[3] = String(item.number); break;
    case 'rom': params[3] = String(100 + item.number); break;
    default: throw new Error();
  }
  params[4] = String(item.flag);
  return `<OBJECT 1,${params.join(',')}>`;
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

function toIntegerItemType(stringItemType: 'subWeapon' | 'equipment' | 'rom') {
  return stringItemType === 'subWeapon' ? 0
    : stringItemType === 'equipment' ? 1
      : stringItemType === 'rom' ? 2
        : (() => { throw new Error(); })();
}
