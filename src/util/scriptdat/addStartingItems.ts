import { EquipmentNumber, SubWeaponNumber } from '../../model/randomizer/items';

export default function addStartingItems(
  txt: string,
  equipmentList: EquipmentNumber[],
  subWeaponList: SubWeaponNumber[],
) {
  const unusedOneTimeFlagNo = 7400;
  const unusedSaveFlagNo = 6000;
  let targetSeek = 0;
  return txt.split('\n').map((x) => {
    if (targetSeek === 0 && x === '<FIELD 1,1,1,1,0>') {
      targetSeek = 1;
      return x;
    }
    if (targetSeek === 1 && x === '<MAP 3,1,2>') {
      targetSeek = 2;
      return x;
    }
    if (targetSeek === 2 && x === '</MAP>') {
      targetSeek = -1;
      // tslint:disable:no-increment-decrement
      return (
        // tslint:disable-next-line:prefer-template
        `<OBJECT 7,38912,14336,7,999,-1,-1></OBJECT>`
        + `<OBJECT 22,26624,10240,2,2,${unusedOneTimeFlagNo},-1></OBJECT>`
        + subWeaponList.map(y => (
          `<OBJECT 13,26624,10240,${y},0,${unusedSaveFlagNo},-1></OBJECT>`
          + `<OBJECT 13,26624,10240,${y},255,${unusedSaveFlagNo},-1></OBJECT>`
        )).join('')
        + equipmentList.map(y => (
          `<OBJECT 1,26624,14336,${unusedOneTimeFlagNo},${y},${unusedSaveFlagNo},-1></OBJECT>`
        )).join('')
        + x
      );
      // tslint:enable:no-increment-decrement
    }
    return x;
  }).join('\n');
}
