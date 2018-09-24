import assert from 'assert';
import Item from '../../../model/dataset/Item';
import Spot from '../../../model/dataset/Spot';
import Storage from '../../../model/dataset/Storage';
import Supplements from '../../../model/dataset/Supplements';
const {
  nightSurfaceChestCount,
  nightSurfacSealCount,
  nightSurfaceSubWeaponCount,
  trueShrineOfTheMotherSealCount,
} = Supplements;
import {
  equipmentNumbers,
  subWeaponNumbers,
} from '../../../model/randomizer/items';
import ShopItemsData, { ShopItemData } from '../format/ShopItemsData';
import LMObject from './LMObject';
import { toObjectForShutter, toObjectForSpecialChest, toObjectsForChest } from './objectfactory';
import { List, LMWorld } from './Script';

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
        // tslint:disable-next-line:max-func-body-length
        objects: map.objects.map((obj, i) => {
          switch (obj.number) {
            case 77: {
              const item = shuffled.mainWeaponShutters[mainWeaponSpotIdx].item;
              mainWeaponSpotIdx += 1;
              const nextShutterCheckFlag = getNextShutterCheckFlag(map.objects.slice(i + 1));
              return [toObjectForShutter(obj, nextShutterCheckFlag, item)];
            }
            case 13: {
              // TODO: nightSurface
              if (subWeaponSpotIdx >= shuffled.subWeaponShutters.length) {
                const sum = (
                  shuffled.subWeaponShutters.length + nightSurfaceSubWeaponCount
                );
                assert(subWeaponSpotIdx < sum);
                subWeaponSpotIdx += 1;
                return [obj];
              }
              const item = shuffled.subWeaponShutters[subWeaponSpotIdx].item;
              subWeaponSpotIdx += 1;
              if (obj.op1 === subWeaponNumbers.ankhJewel) {
                if (obj.op3 !== 743) {
                  return [toObjectForSpecialChest(obj, item)];
                }
                // gateOfGuidance ankhJewel
                const wallCheckFlag = getNextWallCheckFlag(map.objects.slice(i + 1));
                return [toObjectForShutter(obj, wallCheckFlag, item)];
              }
              const nextShutterCheckFlag = obj.op1 === subWeaponNumbers.pistol
                ? getNextBreakableWallCheckFlag(map.objects.slice(i + 1))
                : getNextShutterCheckFlag(map.objects.slice(i + 1));
              return [toObjectForShutter(obj, nextShutterCheckFlag, item)];
            }
            case 1: {
              if (obj.op2 === -1 || obj.op2 === equipmentNumbers.sweetClothing) {
                return [obj];
              }
              // TODO: nightSurface
              if (chestIdx >= shuffled.chests.length) {
                const sum = shuffled.chests.length + nightSurfaceChestCount;
                assert(chestIdx < sum);
                chestIdx += 1;
                return [obj];
              }
              let item: Item;
              // twinStatue
              if (obj.op1 === 420) {
                item = shuffled.chests[chestIdx - 1].item;
              } else {
                item = shuffled.chests[chestIdx].item;
                chestIdx += 1;
              }
              return toObjectsForChest(obj, item);
            }
            case 71: {
              // TODO: trueShrineOfTheMother
              // TODO: nightSurface
              if (sealChestIdx >= shuffled.sealChests.length) {
                const sum = (
                  shuffled.sealChests.length
                  + trueShrineOfTheMotherSealCount
                  + nightSurfacSealCount
                );
                assert(sealChestIdx < sum, JSON.stringify(obj));
                sealChestIdx += 1;
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
            case 186: {
              // Vimana
              if (obj.starts.length === 1 && obj.starts[0].number === 788) {
                return [new LMObject(
                  obj.number, obj.x, obj.y,
                  obj.op1, obj.op2, obj.op3, obj.op4,
                  [{ number: 891, value: obj.starts[0].value }],
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

function getNextBreakableWallCheckFlag(objs: List<LMObject>) {
  const data = objs.find(x => x.number === 70)!.op4;
  return (data - (data / 10000 | 0) * 10000) / 10 | 0;
}
