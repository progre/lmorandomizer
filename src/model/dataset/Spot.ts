import assert from 'assert';
import Item from './Item';

export default class Spot {
  constructor(
    public readonly type: 'weaponShutter' | 'chest' | 'shop',
    public readonly requirementItems: ReadonlyArray<ReadonlyArray<Item>> | null,
    public readonly talkNumber: number | null,
  ) {
    if (type === 'shop') {
      assert.notEqual(talkNumber, null);
    } else {
      assert.equal(talkNumber, null);
    }
  }

  isReachable(currentItems: ReadonlyArray<Item>) {
    if (this.requirementItems == null) {
      return true;
    }
    return this.requirementItems.some(group => (
      group.every(x => currentItems.some(y => y.name === x.name))
    ));
  }
}
