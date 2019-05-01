import assert from 'assert';
import Item from '../../../model/dataset/Item';
import Spot from '../../../model/dataset/Spot';
import Storage from '../../../model/dataset/Storage';
import {
  EquipmentNumber,
  equipmentNumbers,
  SubWeaponNumber,
} from '../../../model/randomizer/items';
import { parseScriptTxt, stringifyScriptTxt } from '../format/scripttxtparser';
import ShopItemsData from '../format/ShopItemsData';
import addStartingItems from './addStartingItems';
import addObject from './addObject';
import LMObject from './LMObject';
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
  ) {
    this.worlds = addStartingItems(this.worlds, equipmentList, subWeaponList);
  }
  
  addObject(
	field: Number,
	screenx: 0 | 1 | 2 | 3,
	screeny: 0 | 1 | 2 | 3 | 4,
	objects: LMObject[],
  ) {
    this.worlds = addObject(this.worlds, field, screenx, screeny, objects);
  }

  tabletSave() {
      //guidance
      const DO_WARP_FLAG = 1100;
      const WARP_GUI = 1101;
      const WARP_SUR = 1102;
      const WARP_MAU = 1103;
      const WARP_SUN = 1104;
      const WARP_SPR = 1105;
      const WARP_INF = 1106;
      const WARP_EXT = 1107;
      const WARP_TLF = 1108;
      const WARP_END = 1109;
      const WARP_MOM = 1110;
      const WARP_CON = 1111;
      const WARP_GRA = 1112;
      const WARP_MOO = 1113;
      const WARP_GOD = 1114;
      const WARP_BIR = 1115;
      const WARP_TLB = 1116;
      const WARP_DIM = 1117;
      const guisaves = [
          new LMObject(14, 14336, 40960, 200, -1, 185, 0, []),                                  // xelpud
          new LMObject(157, 12288, 36864, 4, 4, 10000, WARP_GUI, []),                           // stoparea -- activate WARP_GUI
          new LMObject(40, WARP_GUI, DO_WARP_FLAG, WARP_SUR, WARP_MAU, WARP_SUN, WARP_SPR, []), // turn off other warp flags
          new LMObject(40, WARP_GUI, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_GUI, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_BIR, []),     // ibid
          new LMObject(40, WARP_GUI, WARP_TLB, WARP_DIM, -1, -1, -1, []),                       // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]),                     // disable warp after save.
      ];
      const surfsaves = [
          new LMObject(87, 26624, 16384, 18, 1, 15, 4,
              [{ number: DO_WARP_FLAG, value: true },
                  { number: WARP_SUR, value: false },
              ]
          ), // warp to center of (unused) screen
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, []), // reset do_warp
          new LMObject(157, 16384,12288,4,4,10000,WARP_SUR,[]), // set WARP_SUR when near tent
      ];
      const gotwarps = [
          
          new LMObject(87, 30720, 16384, 1, 7, 13, 5, []), // warp to surface is unconditional, but lower.
          new LMObject(87, 30720, 14336, 0, 6, 7, 18, [{ number: WARP_GUI, value: true }]), // warp to guidance
      ];
      this.worlds = addObject(this.worlds, 0, 2, 1, guisaves);
      this.worlds = addObject(this.worlds, 1, 3, 1, surfsaves);
      this.worlds = addObject(this.worlds, 18, 1, 0, gotwarps);

      this.talks = this.talks.map(talks => (
          (talks == this.talks[84] ? "P２５\x4clease work." : talks) // set flag 1100 in save prompt
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
