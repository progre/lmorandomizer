import assert from 'assert';
import Item from '../../model/dataset/Item';
import Spot from '../../model/dataset/Spot';
import Storage from '../../model/dataset/Storage';
import {
  equipmentNumbers,
  subWeaponNumbers,
} from '../../model/randomizer/items';
import LMObject from './LMObject';
import { toObjectForShutter, toObjectForSpecialChest, toObjectsForChest } from './objectfactory';
import { List, LMWorld } from './Script';
import ShopItemsData, { ShopItemData } from './ShopItemsData';

export function replaceShops(
  talks: ReadonlyArray<string>,
  shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>,
) {
  return talks.map((oldShopStr, i) => {
    const newShop = shops.find(x => x.spot.talkNumber === i);
    if (newShop == null) {
      return oldShopStr;
    }
    const old = ShopItemsData.parse(oldShopStr);
    const replaced = <[ShopItemData, ShopItemData, ShopItemData]>old.map((item, j) => {
      const newShopItem = newShop.items[j];
      return {
        type: toIntegerItemType(newShopItem.type),
        number: newShopItem.number,
        price: item.price,
        count: newShopItem.count,
        flag: newShopItem.flag,
      };
    });
    return ShopItemsData.stringify(replaced);
  });
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

export function replaceItems(
  worlds: ReadonlyArray<LMWorld>,
  shuffled: Storage,
): ReadonlyArray<LMWorld> {
  let mainWeaponSpotIdx = 0;
  let subWeaponSpotIdx = 0;
  let chestIdx = 0;
  let sealChestIdx = 0;

  return worlds.map(world => ({
    ...world,
    fields: world.fields.map(field => ({
      ...field,
      maps: field.maps.map(map => ({
        ...map,
        objects: map.objects.map((obj, i) => {
          switch (obj.number) {
            case 77: {
              const item = shuffled.mainWeaponShutters[mainWeaponSpotIdx].item;
              mainWeaponSpotIdx += 1;
              const nextShutterCheckFlag = getNextShutterCheckFlag(map.objects.slice(i + 1));
              return [toObjectForShutter(obj, nextShutterCheckFlag, item)];
            }
            case 13: {
              // TODO: night surface
              if (subWeaponSpotIdx >= shuffled.subWeaponShutters.length) {
                return [obj];
              }
              const item = shuffled.subWeaponShutters[subWeaponSpotIdx].item;
              subWeaponSpotIdx += 1;
              if (obj.op1 === subWeaponNumbers.ankhJewel) {
                // TODO: 巨人のアンクジュエルが塞がるやつ
                return [toObjectForSpecialChest(obj, item)];
              }
              const nextShutterCheckFlag = obj.op1 === subWeaponNumbers.pistol
                ? 40
                : getNextShutterCheckFlag(map.objects.slice(i + 1));
              return [toObjectForShutter(obj, nextShutterCheckFlag, item)];
            }
            case 1: {
              if (obj.op2 === -1 || obj.op2 === equipmentNumbers.sweetClothing) {
                return [obj];
              }
              // TODO: night surface
              if (chestIdx >= shuffled.chests.length) {
                return [obj];
              }
              const item = shuffled.chests[chestIdx].item;
              chestIdx += 1;
              return toObjectsForChest(obj, item);
            }
            case 71: {
              // TODO: night surface
              if (sealChestIdx >= shuffled.sealChests.length) {
                return [obj];
              }
              const item = shuffled.sealChests[sealChestIdx].item;
              sealChestIdx += 1;
              return [toObjectForSpecialChest(obj, item)];
            }
            case 140: {
              // mausoleumOfTheGiants ankhJewel
              if (obj.x === 49152 && obj.y === 16384) {
                return [new LMObject(
                  obj.number, obj.x, obj.y,
                  shuffled.subWeaponShutters[subWeaponSpotIdx - 1].item.flag,
                  obj.op2, obj.op3, obj.op4,
                  obj.starts,
                )];
              }
              return [obj];
            }
            default:
              return [obj];
          }
        })
          .reduce((p, c) => p.concat(c), []),
      })),
    })),
  }));
}

function getNextShutterCheckFlag(objs: List<LMObject>) {
  return objs.find(x => x.number === 20)!.op1;
}

function getNextWallCheckFlag(objs: List<LMObject>) {
  return objs.find(x => x.number === 59)!.op3;
}
