import assert from 'assert';
import { Item } from './types';

export default class Spot {
  constructor(
    public readonly type: 'chest' | 'shop',
    public readonly requirements: ReadonlyArray<ReadonlyArray<Item>> | null,
    public readonly talkNumber: number | null,
  ) {
    if (type === 'shop') {
      assert.notEqual(talkNumber, null);
    } else {
      assert.equal(talkNumber, null);
    }
  }

  isReachable(currentItems: ReadonlyArray<Item>) {
    if (this.requirements == null) {
      return true;
    }
    return this.requirements.some(group => (
      group.every(x => currentItems.some(y => y.name === x.name))
    ));
  }
}
