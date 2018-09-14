import assert from 'assert';
import { shopItemDataToText, textToShopData } from './codec';

export default {
  parse(text: string) {
    const data = textToShopData(text);
    assert.equal(data.length, 7 * 3);
    return <[ShopItemData, ShopItemData, ShopItemData]>[...Array(3).keys()]
      .map(x => x * 7)
      .map<ShopItemData>(x => ({
        type: <0 | 1 | 2>(data[x + 0] - 1),
        number: data[x + 1] - 1,
        price: ((data[x + 2] - 1) << 8) + data[x + 3],
        count: data[x + 4] - 1,
        flag: ((data[x + 5] - 1) << 8) + data[x + 6], // 254 * 256 + 255 is no set flag
      }));
  },

  stringify(items: [ShopItemData, ShopItemData, ShopItemData]) {
    assert.equal(items.length, 3);
    const data = Uint8Array.from(
      items.map(x => [
        x.type + 1,
        x.number + 1,
        (x.price >> 8) + 1, x.price % 256,
        x.count + 1,
        (x.flag >> 8) + 1, x.flag % 256,
      ]).reduce((p, c) => p.concat(c), []),
    );
    return shopItemDataToText(data);
  },
};

export interface ShopItemData {
  type: 0 | 1 | 2;
  number: number;
  price: number;
  count: number;
  flag: number;
}
