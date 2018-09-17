import assert from 'assert';
import Item from '../../model/dataset/Item';
import Spot from '../../model/dataset/Spot';
import Storage from '../../model/dataset/Storage';
import { EquipmentNumber, equipmentNumbers, SubWeaponNumber } from '../../model/randomizer/items';
import addStartingItems from './addStartingItems';
import { replaceItems, replaceShops } from './scripteditor';
import { parseScriptTxt, stringifyScriptTxt } from './scripttxtparser';
import ShopItemsData from './ShopItemsData';

export type List<T> = ReadonlyArray<Readonly<T>>;

export interface LMWorld {
  value: number;
  fields: List<LMField>;
}

export interface LMField {
  attrs: List<number>;
  children: List<LMChild>;
  objects: List<LMObject>;
  maps: List<LMMap>;
}

export interface LMMap {
  attrs: List<number>;
  children: List<LMChild>;
  objects: List<LMObject>;
}

export interface LMObject {
  number: number;
  x: number;
  y: number;
  op1: number;
  op2: number;
  op3: number;
  op4: number;
  starts: List<LMStart>;
}

export interface LMStart {
  number: number;
  value: boolean;
}

export interface LMChild {
  name: string;
  attrs: List<number>;
}

export default class Script {
  static parse(txt: string) {
    const { talks, worlds } = parseScriptTxt(txt);
    const script = new this(talks, worlds);
    assert.equal(txt, script.stringify(), 'stringify mismatch');
    return script;
  }

  private constructor(
    private talks: ReadonlyArray<string>,
    private worlds: ReadonlyArray<LMWorld>,
  ) {
  }

  stringify() {
    return stringifyScriptTxt(this.talks, this.worlds);
  }

  mainWeapons() {
    return this.viewObjects()
      .filter(x => x.number === 77)
      .map(x => ({ mainWeaponNumber: x.op1, flag: x.op2 }));
  }

  subWeapons() {
    return this.viewObjects()
      .filter(x => x.number === 13)
      .map(x => ({
        subWeaponNumber: x.op1,
        count: x.op2,
        flag: x.op3,
      }));
  }

  chests() {
    return this.viewObjects()
      .filter(x => !(// without 2nd twinStatue
        x.number === 1
        && x.x === 8192
        && x.y === 6144
        && x.op1 === 420
        && x.op2 === 14
        && x.op3 === 766
        && x.op4 === 0
      ))
      .filter(x => x.number === 1)
      .map(x => ({
        chestItemNumber: x.op2,
        flag: x.op3,
      }))
      .filter(({ chestItemNumber }) => (
        chestItemNumber !== -1
        && chestItemNumber !== equipmentNumbers.sweetClothing
      ));
  }

  seals() {
    return this.viewObjects()
      .filter(x => x.number === 71)
      .map(x => ({
        sealNumber: x.op1,
        flag: x.op2,
      }));
  }

  shops() {
    assert(this.talks.every(x => x != null));
    return this.viewObjects()
      .filter(x => x.number === 14 && x.op1 <= 99)
      .map(x => ({
        talkNumber: x.op4,
        talking: this.talks[x.op3],
        items: ShopItemsData.parse(this.talks[x.op4]),
      }));
  }

  replaceShops(shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>) {
    this.talks = replaceShops(this.talks, shops);
  }

  replaceItems(shuffled: Storage) {
    this.worlds = replaceItems(this.worlds, shuffled);
  }

  addStartingItems(
    equipmentList: EquipmentNumber[],
    subWeaponList: SubWeaponNumber[],
  ) {
    this.worlds = addStartingItems(this.worlds, equipmentList, subWeaponList);
  }

  private viewObjects() {
    return this.worlds[0]
      .fields
      .map(x => x.maps.map(y => y.objects))
      .reduce((p, c) => p.concat(c), [])
      .reduce((p, c) => p.concat(c), []);
  }
}
