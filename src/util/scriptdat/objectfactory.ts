import assert from 'assert';
import Item from '../../model/dataset/Item';
import { subWeaponNumbers } from '../../model/randomizer/items';
import LMObject, { LMStart } from './LMObject';

export function toObjectForShutter(
  oldObj: LMObject,
  startFlag: number,
  item: Item,
) {
  switch (item.type) {
    case 'mainWeapon':
      return createMainWeapon(oldObj, item);
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return createSubWeapon(oldObj, item);
    case 'equipment':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return new LMObject(
        1, oldObj.x, oldObj.y, 40, item.number, item.flag, -1,
        [
          { number: 99999, value: true },
          { number: startFlag, value: true },
          ...oldObj.starts.filter(x => (
            x.number !== 99999
            && x.number !== oldObj.getItemFlag()
          )),
        ],
      );
    case 'rom':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return new LMObject(
        1, oldObj.x, oldObj.y, 40, 100 + item.number, item.flag, -1,
        [
          { number: 99999, value: true },
          { number: startFlag, value: true },
          ...oldObj.starts.filter(x => (
            x.number !== 99999
            && x.number !== oldObj.getItemFlag()
          )),
        ],
      );
    case 'seal':
      return createSeal(oldObj, item);
    default: throw new Error();
  }
}

export function toObjectForSpecialChest(oldObj: LMObject, item: Item) {
  switch (item.type) {
    case 'mainWeapon':
      return createMainWeapon(oldObj, item);
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return createSubWeapon(oldObj, item);
    case 'equipment':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return new LMObject(
        1, oldObj.x, oldObj.y, 40, item.number, item.flag, -1,
        oldObj.starts.filter(x => x.number !== oldObj.getItemFlag()),
      );
    case 'rom':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return new LMObject(
        1, oldObj.x, oldObj.y, 40, 100 + item.number, item.flag, -1,
        oldObj.starts.filter(x => x.number !== oldObj.getItemFlag()),
      );
    case 'seal':
      return createSeal(oldObj, item);
    default: throw new Error();
  }
}

export function toObjectsForChest(oldObj: LMObject, item: Item): ReadonlyArray<LMObject> {
  switch (item.type) {
    case 'mainWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return createMainWeaponChest(oldObj, item);
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return createSubWeaponChest(oldObj, item);
    case 'equipment':
      return [new LMObject(
        1, oldObj.x, oldObj.y, oldObj.op1, item.number, item.flag, -1,
        oldObj.starts
          .filter(x => x.number !== oldObj.getItemFlag())
          .concat((// ex.talisman
            oldObj.starts.some(x => x.number === oldObj.getItemFlag())
              ? { number: item.flag, value: false }
              : []
          )),
      )];
    case 'rom':
      return [new LMObject(
        1, oldObj.x, oldObj.y, oldObj.op1, 100 + item.number, item.flag, -1,
        oldObj.starts
          .filter(x => x.number !== oldObj.getItemFlag())
          .concat((
            oldObj.starts.some(x => x.number === oldObj.getItemFlag())
              ? { number: item.flag, value: false }
              : []
          )),
      )];
    case 'seal':
      return createSealChest(oldObj, item);
    default: throw new Error();
  }
}

function createMainWeapon(oldObj: LMObject, item: Item) {
  return new LMObject(
    77, oldObj.x, oldObj.y, item.number, item.flag, -1, -1,
    oldObj.starts
      .filter(x => x.number !== oldObj.getItemFlag())
      .concat({ number: item.flag, value: false }),
  );
}

function createMainWeaponChest(oldObj: LMObject, item: Item) {
  const starts = createChestStarts(oldObj, item.flag);
  return [
    createEmptyChest(oldObj, starts),
    new LMObject(
      77, oldObj.x, oldObj.y, item.number, item.flag, -1, -1,
      starts,
    ),
  ];
}

function createSubWeapon(oldObj: LMObject, item: Item) {
  return new LMObject(
    13, oldObj.x, oldObj.y, item.number, item.count, item.flag, -1,
    oldObj.starts
      .filter(x => x.number !== oldObj.getItemFlag())
      .concat({ number: item.flag, value: false }),
  );
}

function createSubWeaponChest(oldObj: LMObject, item: Item) {
  const starts = createChestStarts(oldObj, item.flag);
  return [
    createEmptyChest(oldObj, starts),
    new LMObject(
      13, oldObj.x, oldObj.y, item.number, item.count, item.flag, -1,
      starts,
    ),
  ];
}

function createSeal(oldObj: LMObject, item: Item) {
  return new LMObject(
    71, oldObj.x, oldObj.y, item.number, item.flag, -1, -1,
    oldObj.starts
      .filter(x => x.number !== oldObj.getItemFlag())
      .concat({ number: item.flag, value: false }),
  );
}

function createSealChest(oldObj: LMObject, item: Item) {
  const starts = createChestStarts(oldObj, item.flag);
  return [
    createEmptyChest(oldObj, starts),
    new LMObject(
      71, oldObj.x, oldObj.y, item.number, item.flag, -1, -1, starts,
    ),
  ];
}

function createEmptyChest(oldObj: LMObject, starts: ReadonlyArray<LMStart>) {
  return new LMObject(
    1, oldObj.x, oldObj.y, oldObj.op1, -1, oldObj.op1, -1, starts,
  );
}

function createChestStarts(oldObj: LMObject, flag: number) {
  return [
    { number: 99999, value: true },
    { number: oldObj.asChestItem().openFlag, value: true },
    { number: flag, value: false },
    ...oldObj.starts.filter(x => (
      x.number !== 99999
      && x.number !== oldObj.getItemFlag()
    )),
  ];
}
