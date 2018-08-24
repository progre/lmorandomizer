import { decode, encode } from './codec';
// import { Item } from './items';

export async function randomize(src: ArrayBuffer, config: {}) {
  let txt = await decode(src);

  txt = randomizeItems(txt);
  // txt = addStartingItems(
  //   txt,
  //   [
  //     ...`${Item.sacredOrb},`.repeat(5).split(',').map(Number),
  //     Item.feather, Item.grappleClaw, Item.boots,
  //   ],
  //   [3],
  // );

  return encode(txt);
}

function randomizeItems(txt: string) {
  const items = shuffle(findItems(txt));
  let itemsIdx = 0;
  return txt.split('\n').map((x) => {
    if (!x.startsWith('<OBJECT 1,')) {
      return x;
    }
    const params = x.slice('<OBJECT 1,'.length, x.length - 1).split(',');
    // アンクジュエル、印、手前より開けは無視
    if (params[3] === '-1') {
      return x;
    }
    const itemData = items[itemsIdx];
    params[3] = itemData.item;
    params[4] = itemData.flag;
    itemsIdx += 1;
    return `<OBJECT 1,${params.join(',')}>`;
  }).join('\n');
}

function findItems(txt: string) {
  return txt.split('\n')
    .filter(x => x.startsWith('<OBJECT 1,'))
    .map(x => x.slice('<OBJECT 1,'.length, x.length - 1).split(','))
    .filter(x => x[3] !== '-1')
    .map(x => ({ item: x[3], flag: x[4] }));
}

function shuffle<T>(list: ReadonlyArray<T>) {
  const array = [...list];
  for (let i = array.length - 1; i >= 0; i -= 1) {
    // tslint:disable-next-line:insecure-random
    const rand = Math.floor(Math.random() * (i + 1));
    [array[i], array[rand]] = [array[rand], array[i]];
  }
  return array;
}

// function addStartingItems(txt: string, items: number[], subWeapons: number[]) {
//   const unusedOneTimeFlagNo = 7400;
//   const unusedSaveFlagNo = 6000;
//   let targetSeek = 0;
//   return txt.split('\n').map((x) => {
//     if (targetSeek === 0 && x === '<FIELD 1,1,1,1,0>') {
//       targetSeek = 1;
//       return x;
//     }
//     if (targetSeek === 1 && x === '<MAP 3,1,2>') {
//       targetSeek = 2;
//       return x;
//     }
//     if (targetSeek === 2 && x === '</MAP>') {
//       targetSeek = -1;
//       // tslint:disable:no-increment-decrement
//       return (
//         // tslint:disable-next-line:prefer-template
//         `<OBJECT 7,38912,14336,7,999,-1,-1></OBJECT>`
//         + `<OBJECT 22,26624,10240,2,2,${unusedOneTimeFlagNo},-1></OBJECT>`
//         + subWeapons.map(y => (
//           `<OBJECT 13,26624,10240,${y},0,${unusedSaveFlagNo},-1></OBJECT>`
//           + `<OBJECT 13,26624,10240,${y},255,${unusedSaveFlagNo},-1></OBJECT>`
//         )).join('')
//         + items.map(y => (
//           `<OBJECT 1,26624,14336,${unusedOneTimeFlagNo},${y},${unusedSaveFlagNo},-1></OBJECT>`
//         )).join('')
//         + x
//       );
//       // tslint:enable:no-increment-decrement
//     }
//     return x;
//   }).join('\n');
// }
