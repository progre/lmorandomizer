import assert from 'assert';
import { prng } from 'seedrandom';
import { decode, encode } from './codec';
import { createItems, createPlaces } from './definitions/factory';
import {
  EquipmentNumber,
  equipmentNumbers,
  SubWeaponNumber,
  subWeaponNumbers,
} from './definitions/items';
import validate from './validate';

export async function randomize(src: ArrayBuffer, config: { rng: prng }) {
  let txt = await decode(src);

  txt = randomizeItems(txt, config.rng);
  if ((<any>0) === 1) {
    txt = addStartingItems(
      txt,
      [
        // ...`${items.sacredOrb},`.repeat(5).split(',').map(Number),
        equipmentNumbers.feather,
        equipmentNumbers.grappleClaw,
        equipmentNumbers.boots,
        equipmentNumbers.serpentStaff,
      ],
      [
        subWeaponNumbers.pistol, subWeaponNumbers.ammunition,
      ],
    );
  }

  return encode(txt);
}

function randomizeItems(txt: string, rng: prng) {
  const source = getSource(txt);
  assert(validate(source));
  let shuffled;
  for (let i = 0; i < 10000; i += 1) {
    // itemをshuffleしてplaceと合わせる
    const s = shuffle(source.map(x => x.item), rng)
      .map((item, j) => ({ item, place: source[j].place }));
    if (s.every(x => x.item.payload.num !== 30)) {
      throw new Error();
    }
    if (validate(s)) {
      assert(s[6].place.payload.conditionGroups[0][0].payload === 30);
      assert(s[6].item.payload.num !== 30);
      shuffled = s;
      break;
    }
  }
  if (shuffled == null) {
    throw new Error();
  }

  const shuffledItems = shuffled.map(x => x.item);
  let idx = 0;
  return txt.split('\n').map((x) => {
    if (idx >= shuffledItems.length) {
      return x;
    }
    if (!x.startsWith('<OBJECT 1,')) {
      return x;
    }
    const params = x.slice('<OBJECT 1,'.length, x.length - 1).split(',');
    // アンクジュエル、印、手前より開けは無視
    if (
      params[3] === '-1'
      || params[3] === String(equipmentNumbers.twinStatue)
      || params[3] === String(equipmentNumbers.sweetClothing)
    ) {
      return x;
    }
    const item = shuffledItems[idx];
    switch (item.type) {
      case 'equipment': params[3] = String(item.payload.num); break;
      case 'rom': params[3] = String(100 + item.payload.num); break;
      default: throw new Error();
    }
    params[4] = String(item.payload.flag);
    idx += 1;
    return `<OBJECT 1,${params.join(',')}>`;
  }).join('\n');
}

function shuffle<T>(list: ReadonlyArray<T>, rng: prng): ReadonlyArray<T> {
  const array = [...list];
  for (let i = array.length - 1; i >= 0; i -= 1) {
    // tslint:disable-next-line:insecure-random
    const rand = Math.floor(rng() * (i + 1));
    [array[i], array[rand]] = [array[rand], array[i]];
  }
  return array;
}

function addStartingItems(
  txt: string,
  equipmentList: EquipmentNumber[],
  subWeaponList: SubWeaponNumber[],
) {
  const unusedOneTimeFlagNo = 7400;
  const unusedSaveFlagNo = 6000;
  let targetSeek = 0;
  return txt.split('\n').map((x) => {
    if (targetSeek === 0 && x === '<FIELD 1,1,1,1,0>') {
      targetSeek = 1;
      return x;
    }
    if (targetSeek === 1 && x === '<MAP 3,1,2>') {
      targetSeek = 2;
      return x;
    }
    if (targetSeek === 2 && x === '</MAP>') {
      targetSeek = -1;
      // tslint:disable:no-increment-decrement
      return (
        // tslint:disable-next-line:prefer-template
        `<OBJECT 7,38912,14336,7,999,-1,-1></OBJECT>`
        + `<OBJECT 22,26624,10240,2,2,${unusedOneTimeFlagNo},-1></OBJECT>`
        + subWeaponList.map(y => (
          `<OBJECT 13,26624,10240,${y},0,${unusedSaveFlagNo},-1></OBJECT>`
          + `<OBJECT 13,26624,10240,${y},255,${unusedSaveFlagNo},-1></OBJECT>`
        )).join('')
        + equipmentList.map(y => (
          `<OBJECT 1,26624,14336,${unusedOneTimeFlagNo},${y},${unusedSaveFlagNo},-1></OBJECT>`
        )).join('')
        + x
      );
      // tslint:enable:no-increment-decrement
    }
    return x;
  }).join('\n');
}

function getSource(txt: string) {
  const items = createItems(txt);
  const places = createPlaces();
  assert(items.length === places.length);
  return items.map((item, i) => ({ item, place: places[i] }));
}
