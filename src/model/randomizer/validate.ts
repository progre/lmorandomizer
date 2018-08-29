import assert from 'assert';
import { PhysicalCondition, PhysicalConditionGroups } from './definitions/conditions';
import { EquipmentNumber, equipmentNumbers, Item } from './definitions/items';
import { Place } from './definitions/places';

export default function validate(game: ReadonlyArray<{ item: Item; place: Place }>) {
  let currentItems: ReadonlyArray<Item> = [
    ...[...Array(7).keys()]
      .map(num => ({
        type: <'mainWeapon'>'mainWeapon',
        payload: { num, flag: -1 },
      })),
    ...[...Array(13).keys()]
      .map(num => ({
        type: <'subWeapon'>'subWeapon',
        payload: { num, flag: -1 },
      })),
    ...[...Array(60).keys()]
      .filter(x => !includesListEquipNum(game, x))
      .map(num => ({
        type: <'equipment'>'equipment',
        payload: { num, flag: -1 },
      })),
    ...[...Array(84).keys()]
      .filter(x => !includesListEquipNum(game, x))
      .map(num => ({
        type: <'rom'>'rom',
        payload: { num, flag: -1 },
      })),
  ];
  let playing = [...game];
  for (; ;) {
    const reached = playing
      .filter(x => (
        isReachable(x.place.payload.conditionGroups, currentItems)
      ))
      .map(x => x.item);
    if (reached.length <= 0) {
      return false;
    }
    playing = playing.filter(x => (
      !isReachable(x.place.payload.conditionGroups, currentItems)
    ));
    if (playing.length <= 0) {
      assert(!hasSelfLock(game));
      return true;
    }
    currentItems = [
      ...currentItems,
      ...reached,
    ];
  }
}

function hasSelfLock(game: ReadonlyArray<{ item: Item; place: Place }>) {
  return game
    // 生命の宝珠は複数あるのでチェックしない
    .filter(x => (
      x.item.type !== 'equipment'
      || x.item.payload.num !== equipmentNumbers.sacredOrb
    ))
    .some(x => (
      x.place.payload.conditionGroups.length === 1
      && x.place.payload.conditionGroups[0].some(y => isSameItem(x.item, y))
    ));
}

function includesListEquipNum(
  list: ReadonlyArray<{ item: Item; place: Place }>,
  equipNum: EquipmentNumber,
) {
  return list.some(
    ({ item }) => item.type === 'equipment' && equipNum === item.payload.num,
  );
}

function includesItemsCondition(
  list: ReadonlyArray<Item>,
  condition: PhysicalCondition,
) {
  return list.some(x => isSameItem(x, condition));
}

function isReachable(
  conditionGroups: PhysicalConditionGroups,
  currentItems: ReadonlyArray<Item>,
) {
  if (conditionGroups.length === 0) {
    return true;
  }
  return conditionGroups.some(group => (
    group.every(x => includesItemsCondition(currentItems, x))
  ));
}

function isSameItem(item: Item, cond: PhysicalCondition) {
  return item.type === cond.type && item.payload.num === cond.payload;
}
