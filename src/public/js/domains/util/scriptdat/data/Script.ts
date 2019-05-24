import assert from 'assert';
import Item from '../../../model/dataset/Item';
import Spot from '../../../model/dataset/Spot';
import Storage from '../../../model/dataset/Storage';
import {
  EquipmentNumber,
  equipmentNumbers,
  SubWeaponNumber,
  subWeaponNumbers,
  romNumbers,
} from '../../../model/randomizer/items';
import { parseScriptTxt, stringifyScriptTxt } from '../format/scripttxtparser';
import ShopItemsData from '../format/ShopItemsData';
import addStartingItems from './addStartingItems';
import LMObject from './LMObject';
import addObject from './addObject';
import tabletSave from './tabletSave';
import { replaceItems, replaceShops } from './scripteditor';

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
      .map(x => x.asMainWeapon());
  }

  subWeapons() {
    return this.viewObjects()
      .filter(x => x.number === 13)
      .map(x => x.asSubWeapon());
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
      .map(x => x.asChestItem())
      .filter(({ chestItemNumber }) => (
        chestItemNumber !== -1
        && chestItemNumber !== equipmentNumbers.sweetClothing
      ));
  }

  seals() {
    return this.viewObjects()
      .filter(x => x.number === 71)
      .map(x => x.asSeal());
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
    easyMode: boolean,
    grail: boolean,
    scanner: boolean,
    gameMaster: boolean,
    glyphReader: boolean,
  ) {
    if (!easyMode) {
      if (grail) {
        equipmentList.push(equipmentNumbers.holyGrail);
      }
      if (gameMaster) {
        equipmentList.push(romNumbers.gameMaster + 100);
      }
      if (glyphReader) {
        equipmentList.push(romNumbers.glyphReader + 100);
      }
      if (scanner) {
        subWeaponList.push(subWeaponNumbers.handScanner);
      }
    }
    this.worlds = addStartingItems(this.worlds, equipmentList, subWeaponList, easyMode);
  }
  
  addObject(
	field: Number,
	screenx: 0 | 1 | 2 | 3,
	screeny: 0 | 1 | 2 | 3 | 4,
	objects: LMObject[],
  ) {
    this.worlds = addObject(this.worlds, field, screenx, screeny, objects);
  }

  tabletSave(easyMode : boolean) {
      //guidance
      this.worlds = tabletSave(this.worlds, easyMode);
      this.talks = this.talks.map(talks => (
          (talks == this.talks[84] ? "２５\x4c".concat(this.talks[84]) : talks) // set flag 1100 in save prompt
      ));

  }

  private viewObjects() {
    return this.worlds[0]
      .fields
      .map(x => x.maps.map(y => y.objects))
      .reduce((p, c) => p.concat(c), [])
      .reduce((p, c) => p.concat(c), []);
  }
}
