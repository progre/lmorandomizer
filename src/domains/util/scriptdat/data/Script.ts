import { invoke } from '@tauri-apps/api/core';
import assert from '../../../../assert';
import Item from '../../../model/dataset/Item';
import Spot from '../../../model/dataset/Spot';
import Storage from '../../../model/dataset/Storage';
import {
  EquipmentNumber,
  equipmentNumbers,
  SubWeaponNumber,
} from '../../../model/randomizer/items';
import ShopItemsData from '../format/ShopItemsData';
import addStartingItems from './addStartingItems';
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
  static async parse(txt: string) {
    let [talks, worlds]: [string[], LMWorld[]] = <any>await invoke('parse_script_txt', { text: txt });
    worlds = worlds.map((world): LMWorld => ({
      ...world,
      fields: world.fields.map((field) => ({
        ...field,
        maps: field.maps.map((map): LMMap => ({
          ...map,
          objects: map.objects.map(LMObject.fromObject),
        })),
        objects: field.objects.map(LMObject.fromObject),
      })),
    }));
    const script = new this(talks, worlds);
    assert.equal(txt, await script.stringify(), 'stringify mismatch');
    return script;
  }

  private constructor(
    private talks: ReadonlyArray<string>,
    private worlds: ReadonlyArray<LMWorld>,
  ) {
  }

  async stringify() {
    return invoke('stringify_script_txt', { talks: this.talks, worlds: this.worlds });
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

  private viewObjects() {
    return this.worlds[0]
      .fields
      .map(x => x.maps.map(y => y.objects))
      .reduce((p, c) => p.concat(c), [])
      .reduce((p, c) => p.concat(c), []);
  }
}
