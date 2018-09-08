import assert from 'assert';
import Item from '../../model/dataset/Item';
import Spot from '../../model/dataset/Spot';
import { Storage } from '../../model/dataset/types';
import { EquipmentNumber, equipmentNumbers, SubWeaponNumber } from '../../model/randomizer/items';
import addStartingItems from './addStartingItems';
import ShopItemsData from './ShopItemsData';
import { replaceChests, replaceShops } from './utils';

export default class ScriptDat {
  constructor(
    public txt: string,
  ) {
  }

  // mainWeapons: txt.split('\n')
  //   .filter(x => x.startsWith('<OBJECT 77,'))
  //   .map(x => x.slice('<OBJECT 77,'.length, x.length - 1).split(','))
  //   .map(x => ({ mainWeaponNumber: Number(x[2]), flag: Number(x[3]) })),
  // subWeapons: txt.split('\n')
  //   .filter(x => x.startsWith('<OBJECT 13,'))
  //   .map(x => x.slice('<OBJECT 13,'.length, x.length - 1).split(','))
  //   .map(x => ({ subweaponNumber: Number(x[2]), flag: Number(x[4]) })),

  chests() {
    return this.txt.split('\n')
      .filter(x => x.startsWith('<OBJECT 1,'))
      .map((x) => {
        const array = x.slice('<OBJECT 1,'.length, x.length - 1).split(',');
        return {
          source: x,
          chestItemNumber: Number(array[3]),
          flag: Number(array[4]),
        };
      })
      .filter(({ chestItemNumber }) => (
        chestItemNumber !== -1
        && chestItemNumber !== equipmentNumbers.twinStatue // TODO: 双子像対応
        && chestItemNumber !== equipmentNumbers.sweetClothing
      ));
  }

  shops() {
    const talks = getTalks(this.txt);
    assert(talks.every(x => x != null));
    return this.txt.split('\n')
      .filter(x => x.startsWith('<OBJECT 14,'))
      .map(x => x.slice('<OBJECT 14,'.length, x.length - 1).split(','))
      .filter(x => Number(x[2]) <= 99)
      .map(x => ({
        talkNumber: Number(x[5]),
        talking: talks[Number(x[4])],
        items: ShopItemsData.parse(talks[Number(x[5])]),
      }));
  }

  addStartingItems(
    equipmentList: EquipmentNumber[],
    subWeaponList: SubWeaponNumber[],
  ) {
    this.txt = addStartingItems(this.txt, equipmentList, subWeaponList);
  }

  replaceChests(shuffled: Storage) {
    this.txt = replaceChests(this.txt, shuffled);
  }

  replaceShops(shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>) {
    this.txt = replaceShops(this.txt, shops);
  }
}

function getTalks(txt: string) {
  const r = '<TALK>\n{}[^]+?{}</TALK>\n'.split('{}');
  const list = txt
    .split(new RegExp(`(${r[0]}${r[1]}${r[2]})`))
    .filter(x => x.length > 0);
  return list
    .slice(0, list.length - 1)
    .map(x => x.split(new RegExp(`${r[0]}(${r[1]})${r[2]}`))[1]);
}
