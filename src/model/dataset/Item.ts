import { equipmentNumbers } from '../randomizer/items';

export default class Item {
  constructor(
    public name: string,
    public type: 'subWeapon' | 'equipment' | 'rom',
    public number: number,
    public count: number,
    public flag: number,
  ) {
  }

  // chests -> equipments / rom
  // chests <- subWeapon / subWeaponAmmo / equipments / rom / sign
  // shops -> equipments / rom
  // shops <- subWeapon / subWeaponAmmo / equipments / rom
  canDisplayInShop() {
    return this.flag % 256 !== 0
      && (
        this.type !== 'equipment'
        || this.number !== equipmentNumbers.map
        && this.number !== equipmentNumbers.sacredOrb
      );
  }
}
