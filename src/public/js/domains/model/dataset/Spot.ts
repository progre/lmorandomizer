import assert from 'assert';
import Item from './Item';

export default class Spot {
  constructor(
    public readonly type: 'weaponShutter' | 'chest' | 'shop' | 'sealChest',
    public readonly requirementItems: ReadonlyArray<ReadonlyArray<Item>> | null,
    public readonly talkNumber: number | null,
  ) {
    if (type === 'shop') {
      assert.notEqual(talkNumber, null);
    } else {
      assert.equal(talkNumber, null);
    }
  }

  isReachable(currentItemNames: ReadonlyArray<string>, sacredOrbCount: number) {
    if (this.requirementItems == null) {
      return true;
    }
    return this.requirementItems.some(group => (
      group.every(x => (
        x.name === 'sacredOrb' && x.count <= sacredOrbCount
        || currentItemNames.includes(x.name)
      ))
    ));
  }
}
