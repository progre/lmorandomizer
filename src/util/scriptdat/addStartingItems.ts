import { EquipmentNumber, SubWeaponNumber } from '../../model/randomizer/items';
import { LMWorld } from './Script';

export default function addStartingItems(
  worlds: ReadonlyArray<LMWorld>,
  equipmentList: EquipmentNumber[],
  subWeaponList: SubWeaponNumber[],
): ReadonlyArray<LMWorld> {
  const unusedOneTimeFlagNo = 7400;
  const unusedSaveFlagNo = 6000;
  const startingItems = [
    {
      number: 7, x: 43008, y: 22528,
      op1: 7, op2: 999, op3: -1, op4: -1,
      starts: [],
    },
    {
      number: 22, x: 26624, y: 10240,
      op1: 2, op2: 2, op3: unusedOneTimeFlagNo, op4: -1,
      starts: [],
    },
    ...subWeaponList.map(x => [
      {
        number: 13, x: 26624, y: 10240,
        op1: x, op2: 0, op3: unusedSaveFlagNo, op4: -1,
        starts: [{ number: unusedSaveFlagNo, value: false }],
      },
      {
        number: 13, x: 26624, y: 10240,
        op1: x, op2: 255, op3: unusedSaveFlagNo, op4: -1,
        starts: [{ number: unusedSaveFlagNo, value: false }],
      },
    ]).reduce((p, c) => p.concat(c), []),
    ...equipmentList.map(x => ({
      number: 1, x: 26624, y: 14336,
      op1: unusedOneTimeFlagNo, op2: x, op3: unusedSaveFlagNo, op4: -1,
      starts: [],
    })),
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
