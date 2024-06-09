import assert from '../../../assert';
import Storage from '../dataset/Storage';

export default function validate(storage: Storage) {
  let currentItemNames = (
    storage.allRequirementNames.filter(x => !storage.allItems.map(y => y.name).includes(x))
  );
  assert.deepEqual(currentItemNames, ['sacredOrb']);
  currentItemNames = [];
  let playing = storage;
  for (let i = 0; i < 100; i += 1) {
    const sacredOrbCount = currentItemNames.filter(x => x.startsWith('sacredOrb:')).length;
    const reached = playing.reachableItemNames(currentItemNames, sacredOrbCount);
    if (reached.length <= 0) {
      // console.warn(JSON.stringify(reached), JSON.stringify(playing.allItems.map(x => x.name)));
      return false;
    }
    playing = playing.unreachables(currentItemNames, sacredOrbCount);
    if (playing.allItems.length <= 0) {
      return true;
    }
    currentItemNames = currentItemNames.concat(reached);
  }
  throw new Error();
}
