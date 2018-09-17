import { EquipmentNumber, SubWeaponNumber } from '../../model/randomizer/items';
import { LMWorld } from './Script';
import LMObject from './LMObject';

export default function addStartingItems(
  worlds: ReadonlyArray<LMWorld>,
  equipmentList: EquipmentNumber[],
  subWeaponList: SubWeaponNumber[],
): ReadonlyArray<LMWorld> {
  const unusedOneTimeFlagNo = 7400;
  const unusedSaveFlagNo = 6000;
  const startingItems = [
    new LMObject(7, 43008, 22528, 7, 999, -1, -1, []),
    new LMObject(22, 26624, 10240, 2, 2, unusedOneTimeFlagNo, -1, []),
    ...subWeaponList.map(x => [
      new LMObject(
        13, 26624, 10240, x, 0, unusedSaveFlagNo, -1,
        [{ number: unusedSaveFlagNo, value: false }],
      ),
      new LMObject(
        13, 26624, 10240, x, 255, unusedSaveFlagNo, -1,
        [{ number: unusedSaveFlagNo, value: false }],
      ),
    ]).reduce((p, c) => p.concat(c), []),
    ...equipmentList.map(x => new LMObject(
      1, 26624, 14336, unusedOneTimeFlagNo, x, unusedSaveFlagNo, -1,
      [],
    )),
  ];
  return worlds.map(world => ({
    value: world.value,
    fields: world.fields.map(field => (
      field.attrs[0] !== 1
        ? field
        : {
          ...field,
          maps: field.maps.map(map => (
            !(map.attrs[0] === 3 && map.attrs[1] === 1)
              ? map
              : {
                ...map,
                objects: map.objects.concat(startingItems),
              }
          )),
        }
    )),
  }));
}
