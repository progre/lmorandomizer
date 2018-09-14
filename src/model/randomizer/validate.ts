import assert from 'assert';
import Storage from '../dataset/Storage';

export default function validate(storage: Storage) {
  const allRequirements = storage.allRequirements();
  const allItems = storage.allItems();
  let currentItems
    = allRequirements.filter(x => allItems.every(y => y.name !== x.name));
  assert.equal(currentItems.length, 0);
  let playing = storage;
  for (let i = 0; i < 100; i += 1) {
    const reached = playing.reachableItems(currentItems);
    if (reached.length <= 0) {
      // console.warn(JSON.stringify(currentItems), JSON.stringify(playing));
      return false;
    }
    playing = playing.unreachables(currentItems);
    if (playing.allItems().length <= 0) {
      return true;
    }
    currentItems = [
      ...currentItems,
      ...reached,
    ];
  }
  throw new Error();
}
