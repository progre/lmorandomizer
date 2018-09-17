import assert from 'assert';
import Item from '../../model/dataset/Item';
import { subWeaponNumbers } from '../../model/randomizer/items';
import { LMObject } from './Script';

export function toObjectForShutter(
  oldObj: LMObject,
  startFlag: number,
  item: Item,
): LMObject {
  switch (item.type) {
    case 'mainWeapon':
      return createMainWeapon(oldObj, item);
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return createSubWeapon(oldObj, item);
    case 'equipment':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return {
        number: 1, x: oldObj.x, y: oldObj.y,
        op1: 40, op2: item.number, op3: item.flag, op4: -1,
        starts: [
          { number: 99999, value: true },
          { number: startFlag, value: true },
        ].concat(oldObj.starts),
      };
    case 'rom':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return {
        number: 1, x: oldObj.x, y: oldObj.y,
        op1: 40, op2: 100 + item.number, op3: item.flag, op4: -1,
        starts: [
          { number: 99999, value: true },
          { number: startFlag, value: true },
        ].concat(oldObj.starts),
      };
    case 'seal':
      return createSeal(oldObj, item);
    default: throw new Error();
  }
}

export function toObjectForSpecialChest(oldObj: LMObject, item: Item): LMObject {
  switch (item.type) {
    case 'mainWeapon':
      return createMainWeapon(oldObj, item);
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return createSubWeapon(oldObj, item);
    case 'equipment':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return {
        number: 1, x: oldObj.x, y: oldObj.y,
        op1: 40, op2: item.number, op3: item.flag, op4: -1,
        starts: oldObj.starts,
      };
    case 'rom':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return {
        number: 1, x: oldObj.x, y: oldObj.y,
        op1: 40, op2: 100 + item.number, op3: item.flag, op4: -1,
        starts: oldObj.starts,
      };
    case 'seal':
      return createSeal(oldObj, item);
    default: throw new Error();
  }
}

export function toObjectsForChest(oldObj: LMObject, item: Item): ReadonlyArray<LMObject> {
  switch (item.type) {
    case 'mainWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return [
        {
          number: 77, x: oldObj.x, y: oldObj.y,
          op1: item.number, op2: item.flag, op3: -1, op4: -1,
          starts: [
            { number: 99999, value: true },
            { number: oldObj.op1, value: true },
            { number: item.flag, value: false },
          ].concat(oldObj.starts),
        },
        createEmptyChest(oldObj),
      ];
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return [
        {
          number: 13, x: oldObj.x, y: oldObj.y,
          op1: item.number, op2: item.count, op3: item.flag, op4: -1,
          starts: [
            { number: 99999, value: true },
            { number: oldObj.op1, value: true },
            { number: item.flag, value: false },
          ].concat(oldObj.starts),
        },
        createEmptyChest(oldObj),
      ];
    case 'equipment':
      return [{
        number: 1, x: oldObj.x, y: oldObj.y,
        op1: oldObj.op1, op2: item.number, op3: item.flag, op4: -1,
        starts: oldObj.starts,
      }];
    case 'rom':
      return [{
        number: 1, x: oldObj.x, y: oldObj.y,
        op1: oldObj.op1, op2: 100 + item.number, op3: item.flag, op4: -1,
        starts: oldObj.starts,
      }];
    case 'seal':
      // assert(oldObj.starts.some(z => z.number === 99999 && z.value));
      return [
        {
          number: 71, x: oldObj.x, y: oldObj.y,
          op1: item.number, op2: item.flag, op3: -1, op4: -1,
          starts: [
            { number: 99999, value: true },
            { number: oldObj.op1, value: true },
            { number: item.flag, value: false },
          ].concat(oldObj.starts),
        },
        createEmptyChest(oldObj),
      ];
    default: throw new Error();
  }
}

function createMainWeapon(oldObj: LMObject, item: Item) {
  return {
    number: 77, x: oldObj.x, y: oldObj.y,
    op1: item.number, op2: item.flag, op3: -1, op4: -1,
    starts: oldObj.starts,
  };
}

function createSubWeapon(oldObj: LMObject, item: Item) {
  return {
    number: 13, x: oldObj.x, y: oldObj.y,
    op1: item.number, op2: item.count, op3: item.flag, op4: -1,
    starts: oldObj.starts,
  };
}

function createSeal(oldObj: LMObject, item: Item) {
  return {
    number: 71, x: oldObj.x, y: oldObj.y,
    op1: item.number, op2: item.flag, op3: -1, op4: -1,
    starts: oldObj.starts,
  };
}

function createEmptyChest(oldObj: LMObject): LMObject {
  return {
    number: 1, x: oldObj.x, y: oldObj.y,
    op1: oldObj.op1, op2: -1, op3: oldObj.op1, op4: -1,
    starts: [],
  };
}
