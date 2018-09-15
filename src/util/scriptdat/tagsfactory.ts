import assert from 'assert';
import Item from '../../model/dataset/Item';
import { subWeaponNumbers } from '../../model/randomizer/items';

export function toTagsForShutter(
  x: number,
  y: number,
  startFlag: number,
  item: Item,
) {
  switch (item.type) {
    case 'mainWeapon':
      return `<OBJECT 77,${x},${y},${item.number},${item.flag},-1,-1>`;
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return `<OBJECT 13,${x},${y},${item.number},${item.count},${item.flag},-1>`;
    case 'equipment':
      return (
        `<OBJECT 1,${x},${y},40,${item.number},${item.flag},-1>`
        + '<START 99999,1>'
        + `<START ${startFlag},1>`
      );
    case 'rom':
      return (
        `<OBJECT 1,${x},${y},40,${100 + item.number},${item.flag},-1>`
        + '<START 99999,1>'
        + `<START ${startFlag},1>`
      );
    case 'seal':
      return `<OBJECT 71,${x},${y},${item.number},${item.flag},-1,-1>`;
    default: throw new Error();
  }
}

export function toTagsForChest(x: number, y: number, open: number, item: Item) {
  switch (item.type) {
    case 'mainWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return `<OBJECT 77,${x},${y},${item.number},${item.flag},-1,-1>`
        + '<START 99999,1>'
        + `<START ${open},1>`
        + `<START ${item.flag},0>`
        + `</OBJECT>`
        + `<OBJECT 1,${x},${y},${open},-1,${open},-1>`;
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return `<OBJECT 13,${x},${y},${item.number},${item.count},${item.flag},-1>`
        + '<START 99999,1>'
        + `<START ${open},1>`
        + `<START ${item.flag},0>`
        + `</OBJECT>`
        + `<OBJECT 1,${x},${y},${open},-1,${open},-1>`;
    case 'equipment':
      return `<OBJECT 1,${x},${y},${open},${item.number},${item.flag},-1>`;
    case 'rom':
      return `<OBJECT 1,${x},${y},${open},${100 + item.number},${item.flag},-1>`;
    case 'seal':
      return `<OBJECT 71,${x},${y},${item.number},${item.flag},-1,-1>`
        + '<START 99999,1>'
        + `<START ${open},1>`
        + `<START ${item.flag},0>`
        + `</OBJECT>`
        + `<OBJECT 1,${x},${y},${open},-1,${open},-1>`;
    default: throw new Error();
  }
}

export function toTagsForSealChest(x: number, y: number, item: Item) {
  switch (item.type) {
    case 'mainWeapon':
      return `<OBJECT 77,${x},${y},${item.number},${item.flag},-1,-1>`;
    case 'subWeapon':
      assert(!(item.number === subWeaponNumbers.ankhJewel && item.count > 1));
      return `<OBJECT 13,${x},${y},${item.number},${item.count},${item.flag},-1>`;
    case 'equipment':
      return `<OBJECT 1,${x},${y},40,${item.number},${item.flag},-1>`;
    case 'rom':
      return `<OBJECT 1,${x},${y},40,${100 + item.number},${item.flag},-1>`;
    case 'seal':
      return `<OBJECT 71,${x},${y},${item.number},${item.flag},-1,-1>`;
    default: throw new Error();
  }
}
