import Item from './Item';

export default interface Spot {
  readonly type: 'weaponShutter' | 'chest' | 'shop' | 'sealChest';
  readonly requirementItems: ReadonlyArray<ReadonlyArray<Item>> | null;
  readonly talkNumber: number | null;
}
