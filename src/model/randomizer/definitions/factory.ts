import { CHESTS } from './chests';
import { EquipmentNumber, equipmentNumbers, Item } from './items';
import { Place } from './places';

export function createItems(txt: string): ReadonlyArray<Item> {
  return txt.split('\n')
    .filter(x => x.startsWith('<OBJECT 1,'))
    .map((line) => {
      const params = line.slice('<OBJECT 1,'.length, line.length - 1).split(',');
      const num: EquipmentNumber = Number(params[3]);
      const flag = Number(params[4]);
      if (num < 100) {
        return { type: <'equipment'>'equipment', payload: { num, flag } };
      }
      return { type: <'rom'>'rom', payload: { flag, num: num - 100 } };
    })
    .filter(x => (
      x.type !== 'equipment'
      || (
        equipmentNumbers.msx <= x.payload.num
        && x.payload.num < equipmentNumbers.sweetClothing // あぶねえ水着は対象外
      )
    ))
    .filter((x, i, array) => (
      i === array.findIndex(y => (
        x.type === y.type
        && x.payload.num === y.payload.num
        && x.payload.flag === y.payload.flag
      ))
    ))
    // 双子の像は扱いがややこしいので後回し
    .filter(x => x.type !== 'equipment' || x.payload.num !== equipmentNumbers.twinStatue);
}

export function createPlaces(): ReadonlyArray<Place> {
  return CHESTS.map(payload => ({ payload, type: <'chest'>'chest' }));
}
