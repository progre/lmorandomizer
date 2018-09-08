import { Item, Storage } from '../dataset/types';

export default function validate(storage: Storage) {
  const requirementItems = getAllRequirementsFromItems(
    storage.chests.map(x => x.spot),
  );
  let currentItems
    = requirementItems.filter(x => !storage.chests.map(y => y.item.name).includes(x.name));
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

function getAllRequirementsFromItems(
  items: ReadonlyArray<{ requirements?: ReadonlyArray<ReadonlyArray<Item>> | null }>,
) {
  return items
    .filter(x => x.requirements != null)
    .map(x => x.requirements!.reduce((p, c) => [...p, ...c], []))
    .reduce((p, c) => [...p, ...c], []);
}
