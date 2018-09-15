import assert from 'assert';
import Storage from '../dataset/Storage';

export default async function validate(storage: Storage) {
  let currentItemNames = (
    storage.allRequirementNames.filter(x => !storage.allItems.map(y => y.name).includes(x))
  );
  assert.equal(currentItemNames.length, 0);
  let playing = storage;
  for (let i = 0; i < 100; i += 1) {
    const reached = playing.reachableItemNames(currentItemNames);
    if (reached.length <= 0) {
      // console.warn(JSON.stringify(currentItems), JSON.stringify(playing));
      return false;
    }
    playing = playing.unreachables(currentItemNames);
    if (playing.allItems.length <= 0) {
      return true;
    }
    currentItemNames = currentItemNames.concat(reached);
  }
  throw new Error();
}
