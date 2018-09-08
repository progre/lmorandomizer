import Item from './Item';
import Spot from './Spot';

export interface Storage {
  chests: ReadonlyArray<{ spot: Spot; item: Item }>;
  shops: ReadonlyArray<{ spot: Spot; items: [Item, Item, Item] }>;
}
