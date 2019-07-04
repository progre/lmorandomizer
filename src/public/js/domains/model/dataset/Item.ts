import assert from 'assert';
import { equipmentNumbers, subWeaponNumbers } from '../randomizer/items';

export default class Item {
  constructor(
    public name: string,
    public type: 'mainWeapon' | 'subWeapon' | 'equipment' | 'rom' | 'seal',
    public number: number,
    public count: number,
    public flag: number,
  ) {
    assert(
      flag === -1
      || flag === 494
      || flag === 524
      || 684 <= flag && flag <= 883
      || type === 'subWeapon' && flag === 65279,
      `invalid value: ${flag} (${number})`,
    );
  }

  // chests -> equipments / rom
  // chests <- subWeapon / subWeaponAmmo / equipments / rom / sign
  // shops -> equipments / rom
  // shops <- subWeapon / subWeaponAmmo / equipments / rom
  canDisplayInShop() {
    return (
      this.flag % 256 !== 0
      && (
        this.type === 'equipment'
        // && this.number !== equipmentNumbers.map
        && this.number !== equipmentNumbers.sacredOrb
        || this.type === 'rom'
        || this.type === 'subWeapon'
        && (
          this.count > 0
          || this.number === subWeaponNumbers.pistol
          || this.number === subWeaponNumbers.buckler
          || this.number === subWeaponNumbers.handScanner
          || this.number === subWeaponNumbers.silverShield
          || this.number === subWeaponNumbers.angelShield
        )
      )
    );
  }
}
