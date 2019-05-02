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
      const WARP_RUI = 1115;
      const WARP_BIR = 1116;
      const WARP_TLB = 1117;
      const WARP_DIM = 1118;
      const guisaves = [
          new LMObject(14, 14336, 40960, 200, -1, 185, 0, []),                                  // xelpud
          new LMObject(157, 12288, 36864, 4, 4, 10000, WARP_GUI, []),                           // stoparea -- activate WARP_GUI
          new LMObject(40, WARP_GUI, DO_WARP_FLAG, WARP_SUR, WARP_MAU, WARP_SUN, WARP_SPR, []), // turn off other warp flags
          new LMObject(40, WARP_GUI, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_GUI, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_GUI, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const mausaves = [
          new LMObject(14, 16384, 32768, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 14336, 28672, 4, 4, 10000, WARP_MAU, []), // stoparea -- activate WARP_MAU
          new LMObject(40, WARP_MAU, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_SUN, WARP_SPR, []), // turn off other warp flags
          new LMObject(40, WARP_MAU, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_MAU, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_MAU, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const sunsaves = [
          new LMObject(14, 57344, 16384, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 55296, 12288, 4, 4, 10000, WARP_SUN, []), // stoparea -- activate WARP_SUN
          new LMObject(40, WARP_SUN, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SPR, []), // turn off other warp flags
          new LMObject(40, WARP_SUN, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_SUN, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_SUN, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const sprsaves = [
          new LMObject(14, 14336, 16384, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 12288, 12288, 4, 4, 10000, WARP_SPR, []), // stoparea -- activate WARP_SPR
          new LMObject(40, WARP_SPR, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_SPR, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_SPR, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const infsaves = [
          new LMObject(14, 30720, 8192, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 28672, 4096, 4, 4, 10000, WARP_INF, []), // stoparea -- activate WARP_INF
          new LMObject(40, WARP_INF, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_INF, WARP_SPR, WARP_EXT, WARP_TLF, WARP_END, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_INF, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_INF, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const extsaves = [
          new LMObject(14, 4096, 16384, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 2048, 12288, 4, 4, 10000, WARP_EXT, []), // stoparea -- activate WARP_EXT
          new LMObject(40, WARP_EXT, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_EXT, WARP_SPR, WARP_INF, WARP_TLF, WARP_END, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_EXT, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_EXT, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const tlfsaves = [
          new LMObject(14, 10240, 40960, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 8192, 36864, 4, 4, 10000, WARP_TLF, []), // stoparea -- activate WARP_TLF
          new LMObject(40, WARP_TLF, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_TLF, WARP_SPR, WARP_INF, WARP_EXT, WARP_END, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_TLF, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_TLF, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const endsaves = [
          new LMObject(14, 45056, 8192, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 43008, 4096, 4, 4, 10000, WARP_END, []), // stoparea -- activate WARP_END
          new LMObject(40, WARP_END, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_END, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_MOM, []),     // ibid
          new LMObject(40, WARP_END, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_END, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const momsaves = [
          new LMObject(14, 43008, 8192, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 40960, 4096, 4, 4, 10000, WARP_MOM, []), // stoparea -- activate WARP_MOM
          new LMObject(40, WARP_MOM, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_MOM, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_MOM, WARP_CON, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_MOM, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const consaves = [
          new LMObject(14, 28672, 8192, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 26624, 4096, 4, 4, 10000, WARP_CON, []), // stoparea -- activate WARP_CON
          new LMObject(40, WARP_CON, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_CON, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_CON, WARP_MOM, WARP_GRA, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_CON, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const grasaves = [
          new LMObject(14, 16384, 16384, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 14336, 12288, 4, 4, 10000, WARP_GRA, []), // stoparea -- activate WARP_GRA
          new LMObject(40, WARP_GRA, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_GRA, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_GRA, WARP_MOM, WARP_CON, WARP_MOO, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_GRA, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const moonsaves = [
          new LMObject(14, 22528, 24576, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 20480, 20480, 4, 4, 10000, WARP_MOO, []), // stoparea -- activate WARP_MOO
          new LMObject(40, WARP_MOO, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_MOO, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_MOO, WARP_MOM, WARP_GRA, WARP_CON, WARP_GOD, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_MOO, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const godsaves = [
          new LMObject(14, 30720, 8192, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 28672, 4096, 4, 4, 10000, WARP_GOD, []), // stoparea -- activate WARP_GOD
          new LMObject(40, WARP_GOD, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_GOD, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_GOD, WARP_MOM, WARP_GRA, WARP_CON, WARP_MOO, WARP_RUI, []),     // ibid
          new LMObject(40, WARP_GOD, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const ruisaves = [
          new LMObject(14, 30720, 40960, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 28672, 36864, 4, 4, 10000, WARP_RUI, []), // stoparea -- activate WARP_RUI
          new LMObject(40, WARP_RUI, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_RUI, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_RUI, WARP_MOM, WARP_GRA, WARP_CON, WARP_MOO, WARP_GOD, []),     // ibid
          new LMObject(40, WARP_RUI, WARP_BIR, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const birsaves = [
          new LMObject(14, 59392, 40960, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 253952, 36864, 4, 4, 10000, WARP_BIR, []), // stoparea -- activate WARP_BIR
          new LMObject(40, WARP_BIR, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_BIR, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_BIR, WARP_MOM, WARP_GRA, WARP_CON, WARP_MOO, WARP_GOD, []),     // ibid
          new LMObject(40, WARP_BIR, WARP_RUI, WARP_TLB, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const tlbsaves = [
          new LMObject(14, 59392, 40960, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 253952, 36864, 4, 4, 10000, WARP_TLB, []), // stoparea -- activate WARP_TLB
          new LMObject(40, WARP_TLB, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_TLB, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_TLB, WARP_MOM, WARP_GRA, WARP_CON, WARP_MOO, WARP_GOD, []),     // ibid
          new LMObject(40, WARP_TLB, WARP_RUI, WARP_BIR, WARP_DIM, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const dimsaves = [
          new LMObject(14, 59392, 40960, 200, -1, 185, 0, []), // xelpud
          new LMObject(157, 253952, 36864, 4, 4, 10000, WARP_DIM, []), // stoparea -- activate WARP_TLB
          new LMObject(40, WARP_DIM, DO_WARP_FLAG, WARP_GUI, WARP_SUR, WARP_MAU, WARP_SUN, []), // turn off other warp flags
          new LMObject(40, WARP_DIM, WARP_SPR, WARP_INF, WARP_EXT, WARP_TLF, WARP_END, []),     // ibid
          new LMObject(40, WARP_DIM, WARP_MOM, WARP_GRA, WARP_CON, WARP_MOO, WARP_GOD, []),     // ibid
          new LMObject(40, WARP_DIM, WARP_RUI, WARP_BIR, WARP_TLB, -1, -1, []),                 // ibid
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
      ];
      const surfsaves = [
          new LMObject(87, 26624, 16384, 18, 1, 15, 4,
              [{ number: DO_WARP_FLAG, value: true },
                  { number: WARP_SUR, value: false },
              ]
          ), // warp to center of (unused) screen
          new LMObject(40, DO_WARP_FLAG, DO_WARP_FLAG, -1, -1, -1, -1, [{ number: 99999, value: true }]), // disable warp after save.
          new LMObject(157, 16384,12288,4,4,10000,WARP_SUR,[]), // set WARP_SUR when near tent
      ];
      const gotwarps = [
          new LMObject(87, 30720, 16384, 1, 7, 13, 5, []), // warp to surface is lower. player should not reach this.
          new LMObject(87, 30720, 14336, 0, 6, 7, 19, [{ number: WARP_GUI, value: true }]), // warp to guidance
          new LMObject(87, 30720, 14336, 2, 8, 8, 15, [{ number: WARP_MAU, value: true }]), // warp to mausoleum
          new LMObject(87, 30720, 14336, 3, 2, 28, 7, [{ number: WARP_SUN, value: true }]), // warp to sun
          new LMObject(87, 30720, 14336, 4, 7, 7, 7, [{ number: WARP_SPR, value: true }]), // warp to spring
          new LMObject(87, 30720, 14336, 5, 14, 15, 3, [{ number: WARP_INF, value: true }]), // warp to inferno
          new LMObject(87, 30720, 14336, 6, 19, 2, 7, [{ number: WARP_EXT, value: true }]), // warp to extinction
          new LMObject(87, 30720, 14336, 9, 0, 5, 19, [{ number: WARP_TLF, value: true }]), // warp to twin labyrinths (front)
          new LMObject(87, 30720, 14336, 7, 0, 22, 3, [{ number: WARP_END, value: true }]), // warp to endless corridor
          new LMObject(87, 30720, 14336, 8, 17, 21, 30, [{ number: WARP_MOM, value: true }]), // warp to shrine of the mother
          new LMObject(87, 30720, 14336, 11, 4, 14, 3, [{ number: WARP_CON, value: true }]), //warp to confusion gate
          new LMObject(87, 30720, 14336, 12, 7, 8, 7, [{ number: WARP_GRA, value: true }]), // warp to graveyard
          new LMObject(87, 30720, 14336, 14, 4, 11, 11, [{ number: WARP_MOO, value: true }]), // warp to moonlight
          new LMObject(87, 30720, 14336, 13, 16, 15, 3, [{ number: WARP_GOD, value: true }]), // warp to goddess
          new LMObject(87, 30720, 14336, 15, 4, 15, 19, [{ number: WARP_RUI, value: true }]), // warp to ruin
          new LMObject(87, 30720, 14336, 16, 3, 29, 19, [{ number: WARP_BIR, value: true }]), // warp to birth
          new LMObject(87, 30720, 14336, 10, 3, 25, 19, [{ number: WARP_TLB, value: true }]), // warp to twin labyrinths (back)
          new LMObject(87, 30720, 14336, 17, 10, 9, 11, [{ number: WARP_DIM, value: true }]), // warp to dimensional corridor
      ];
      this.worlds = addObject(this.worlds, 0, 2, 1, guisaves);
      this.worlds = addObject(this.worlds, 1, 3, 1, surfsaves);
      this.worlds = addObject(this.worlds, 2, 0, 2, mausaves);
      this.worlds = addObject(this.worlds, 3, 2, 0, sunsaves);
      this.worlds = addObject(this.worlds, 4, 1, 3, sprsaves);
      this.worlds = addObject(this.worlds, 5, 2, 3, infsaves);
      this.worlds = addObject(this.worlds, 6, 3, 4, extsaves);
      this.worlds = addObject(this.worlds, 7, 0, 0, endsaves); 
      this.worlds = addObject(this.worlds, 8, 1, 4, momsaves);
      this.worlds = addObject(this.worlds, 9, 0, 0, tlfsaves);
      this.worlds = addObject(this.worlds, 10, 3, 0, tlbsaves);
      this.worlds = addObject(this.worlds, 11, 0, 1, consaves);
      this.worlds = addObject(this.worlds, 12, 3, 1, grasaves);
      this.worlds = addObject(this.worlds, 13, 0, 4, godsaves);
      this.worlds = addObject(this.worlds, 14, 0, 1, moonsaves); // not a typo
      this.worlds = addObject(this.worlds, 15, 0, 1, ruisaves);
      this.worlds = addObject(this.worlds, 16, 3, 0, birsaves);
      this.worlds = addObject(this.worlds, 17, 2, 2, dimsaves);

      this.worlds = addObject(this.worlds, 18, 1, 0, gotwarps);
      this.talks = this.talks.map(talks => (
          (talks == this.talks[84] ? "２５\x4c".concat(this.talks[84]) : talks) // set flag 1100 in save prompt
      ));

      this.worlds = addStartingItems(this.worlds, [100], []);

  }

  private viewObjects() {
    return this.worlds[0]
      .fields
      .map(x => x.maps.map(y => y.objects))
      .reduce((p, c) => p.concat(c), [])
      .reduce((p, c) => p.concat(c), []);
  }
}
