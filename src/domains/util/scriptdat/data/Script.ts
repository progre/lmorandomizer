import { invoke } from '@tauri-apps/api/core';
import Item from '../../../model/dataset/Item';
import Spot from '../../../model/dataset/Spot';
import Storage from '../../../model/dataset/Storage';
import {
  EquipmentNumber,
  SubWeaponNumber,
} from '../../../model/randomizer/items';
import { ShopItemData } from '../format/ShopItemsData';
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

  mainWeapons(): Promise<ReadonlyArray<{ mainWeaponNumber: number; flag: number }>> {
    return invoke('script_main_weapons', { this: this });
  }

  subWeapons(): Promise<ReadonlyArray<{ subWeaponNumber: number; count: number; flag: number }>> {
    return invoke('script_sub_weapons', { this: this });
  }

  chests(): Promise<ReadonlyArray<{ chestItemNumber: number; openFlag: number; flag: number }>> {
    return invoke('script_chests', { this: this });
  }

  seals(): Promise<ReadonlyArray<{ sealNumber: number; flag: number }>> {
    return invoke('script_seals', { this: this });
  }

  shops(): Promise<ReadonlyArray<{ talkNumber: number; talking: string; items: ShopItemData[] }>> {
    return invoke('script_shops', { this: this });
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
