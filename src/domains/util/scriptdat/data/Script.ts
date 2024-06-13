import { invoke } from '@tauri-apps/api/core';
import Item from '../../../model/dataset/Item';
import Spot from '../../../model/dataset/Spot';
import Storage from '../../../model/dataset/Storage';
import {
  EquipmentNumber,
  SubWeaponNumber,
} from '../../../model/randomizer/items';
import LMObject from './LMObject';

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
  static from_object(obj: Script) {
    return new Script(obj.talks, obj.worlds);
  }

  private constructor(
    private talks: ReadonlyArray<string>,
    private worlds: ReadonlyArray<LMWorld>,
  ) {
  }

  async replaceShops(shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>) {
    this.talks = (<any>await invoke('script_replace_shops', { this: this, shops })).talks;
  }

  async replaceItems(shuffled: Storage) {
    this.worlds = (<any>await invoke('script_replace_items', { this: this, shuffled })).worlds;
  }

  async addStartingItems(
    equipmentList: EquipmentNumber[],
    subWeaponList: SubWeaponNumber[],
  ) {
    this.worlds = (<any>await invoke('script_add_starting_items', {
      this: this,
      equipmentList,
      subWeaponList,
    })).worlds;
  }
}
