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
      return new LMObject(
        1, oldObj.x, oldObj.y, 40, item.number, item.flag, -1,
        startsThatHideWhenStartup(oldObj, startFlag),
      );
    case 'rom':
      return new LMObject(
        1, oldObj.x, oldObj.y, 40, 100 + item.number, item.flag, -1,
        startsThatHideWhenStartup(oldObj, startFlag),
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
      return new LMObject(
        1, oldObj.x, oldObj.y, 40, item.number, item.flag, -1,
        getStartsWithoutOldFlag(oldObj),
      );
    case 'rom':
      return new LMObject(
        1, oldObj.x, oldObj.y, 40, 100 + item.number, item.flag, -1,
        getStartsWithoutOldFlag(oldObj),
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
        startsAsIs(oldObj, item),
      )];
    case 'rom':
      return [new LMObject(
        1, oldObj.x, oldObj.y, oldObj.op1, 100 + item.number, item.flag, -1,
        startsAsIs(oldObj, item),
      )];
    case 'seal':
      return createSealChest(oldObj, item);
    default: throw new Error();
  }
}

function createMainWeapon(oldObj: LMObject, item: Item) {
  return new LMObject(
    77, oldObj.x, oldObj.y, item.number, item.flag, -1, -1,
    startsAsIs(oldObj, item),
  );
}

function createMainWeaponChest(oldObj: LMObject, item: Item) {
  return [
    createEmptyChest(oldObj, item),
    new LMObject(
      77, oldObj.x, oldObj.y, item.number, item.flag, -1, -1,
      startsThatHideWhenStartupAndTaken(oldObj, item),
    ),
  ];
}

function createSubWeapon(oldObj: LMObject, item: Item) {
  return new LMObject(
    13, oldObj.x, oldObj.y, item.number, item.count, item.flag, -1,
    startsAsIs(oldObj, item),
  );
}

function createSubWeaponChest(oldObj: LMObject, item: Item) {
  return [
    createEmptyChest(oldObj, item),
    new LMObject(
      13, oldObj.x, oldObj.y, item.number, item.count, item.flag, -1,
      startsThatHideWhenStartupAndTaken(oldObj, item),
    ),
  ];
}

function createSeal(oldObj: LMObject, item: Item) {
  return new LMObject(
    71, oldObj.x, oldObj.y, item.number, item.flag, -1, -1,
    startsAsIs(oldObj, item),
  );
}

function createSealChest(oldObj: LMObject, item: Item) {
  return [
    createEmptyChest(oldObj, item),
    new LMObject(
      71, oldObj.x, oldObj.y, item.number, item.flag, -1, -1,
      startsThatHideWhenStartupAndTaken(oldObj, item),
    ),
  ];
}

function createEmptyChest(oldObj: LMObject, item: Item) {
  return new LMObject(
    1, oldObj.x, oldObj.y, oldObj.op1, -1, oldObj.op1, -1,
    startsAsIs(oldObj, item),
  );
}

function startsThatHideWhenStartupAndTaken(oldChestObj: LMObject, item: Item) {
  assert.equal(oldChestObj.number, 1);
  return [
    { number: 99999, value: true },
    { number: oldChestObj.asChestItem().openFlag, value: true },
    { number: item.flag, value: false },
    ...getStartsWithoutOldFlag(oldChestObj).filter(x => x.number !== 99999),
  ];
}

function startsThatHideWhenStartup(oldObj: LMObject, startFlag: number) {
  return [
    { number: 99999, value: true },
    { number: startFlag, value: true },
    ...getStartsWithoutOldFlag(oldObj).filter(x => x.number !== 99999),
  ];
}

function startsAsIs(oldObj: LMObject, item: Item) {
  return [
    ...getStartsWithoutOldFlag(oldObj),
    ...(
      oldObj.starts.some(x => x.number === oldObj.getItemFlag())
        ? [{ number: item.flag, value: false }]
        : []
    ),
  ];
}

function getStartsWithoutOldFlag(oldObj: LMObject) {
  return oldObj.starts.filter(x => x.number !== oldObj.getItemFlag());
}
