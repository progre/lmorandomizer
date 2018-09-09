import Item from '../dataset/Item';
import { Storage } from '../dataset/types';

export default function validate(storage: Storage) {
  const requirements = getAllRequirements(storage);
  let currentItems
    = requirements.filter(x => storage.chests.every(y => y.item.name !== x.name));
  let playing = storage;
  for (; ;) {
    const reached = playing.chests
      .filter(x => x.spot.isReachable(currentItems))
      .map(x => x.item);
    if (reached.length <= 0) {
      return false;
    }
    playing = {
      ...playing,
      chests: playing.chests.filter(x => !x.spot.isReachable(currentItems)),
    };
    if (playing.chests.length <= 0) {
      return true;
    }
    currentItems = [
      ...currentItems,
      ...reached,
    ];
  }
}

function getAllRequirements(storage: Storage) {
  return [...new Set([
    // ...getAllRequirementsFromItems(this.mainWeapons),
    // ...getAllRequirementsFromItems(this.subWeapons),
    ...getAllRequirementsFromItems(storage.chests.map(x => x.spot)),
    ...getAllRequirementsFromItems(storage.shops.map(x => x.spot)),
  ])].sort();
}

function getAllRequirementsFromItems(
  items: ReadonlyArray<{ requirementItems: ReadonlyArray<ReadonlyArray<Item>> | null }>,
) {
  return items
    .filter(x => x.requirementItems != null)
    .map(x => x.requirementItems!.reduce((p, c) => [...p, ...c], []))
    .reduce((p, c) => [...p, ...c], []);
}
