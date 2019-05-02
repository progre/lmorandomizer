import { EquipmentNumber, SubWeaponNumber } from '../../../model/randomizer/items';
import LMObject from './LMObject';
import { LMWorld } from './Script';
import Flags from './Flags';
import addObject from './addObject';

export default function addStartingItems(
  worlds: ReadonlyArray<LMWorld>,
  equipmentList: EquipmentNumber[],
  subWeaponList: SubWeaponNumber[],
): ReadonlyArray<LMWorld> {
  const startingItems = [
    new LMObject(7, 43008, 22528, 7, 46, -1, -1, []), // money
    new LMObject(7, 43008, 22528, 6, 3, -1, -1, []), // weights
    new LMObject(22, 26624, 10240, 2, 2, Flags.EASY_RESPAWN, -1, []),
    ...subWeaponList.map(x => [
      new LMObject(
        13, 26624, 10240, x, 0, Flags.EASY_SAVE, -1,
        [{ number: Flags.EASY_SAVE, value: false }],
      ),
      new LMObject(
        13, 26624, 10240, x, 255, Flags.EASY_SAVE, -1,
        [{ number: Flags.EASY_SAVE, value: false }],
      ),
    ]).reduce((p, c) => p.concat(c), []),
    ...equipmentList.map(x => new LMObject(
      1, 26624, 14336, Flags.EASY_RESPAWN, x, Flags.EASY_SAVE, -1,
      [],
    )),
  ];
  return addObject(worlds, 1, 3, 1, startingItems);
}
