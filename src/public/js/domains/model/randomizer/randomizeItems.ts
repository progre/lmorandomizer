import assert from 'assert';
import seedrandom, { prng } from 'seedrandom';
import Script from '../../util/scriptdat/data/Script';
import createSource from '../dataset/createSource';
import Item from '../dataset/Item';
import Spot from '../dataset/Spot';
import Storage from '../dataset/Storage';
import Supplements from '../dataset/Supplements';
import { selectRandom, shuffleSimply } from './shuffleUtils';
import validate from './validate';

export default function randomizeItems(
  script: Script,
  supplements: Supplements,
  seed: string,
) {
  const source = createSource(script, supplements);
  assert(validate(source));
  assertUnique(source);
  const shuffled = randomizeStorage(source, seedrandom(seed));
  assertUnique(shuffled);
  script.replaceItems(shuffled);
  script.replaceShops(shuffled.shops);
}

function randomizeStorage(source: Storage, rng: prng) {
  let shuffled;
  for (let i = 0; i < 10000; i += 1) {
    // itemをshuffleしてplaceと合わせる
    const storage = shuffle(source, rng);
    if (validate(storage)) {
      shuffled = storage;
      break;
    }
  }
  if (shuffled == null) {
    throw new Error();
  }
  return shuffled;
}

function shuffle(source: Storage, rng: prng): Storage {
  const allItems = source.allItems;
  const {
    newMainWeaponShutters,
    newSubWeaponShutters,
    newChestItems,
    newSealChestItems,
    newShopItems,
  } = distributeItems(allItems, source, rng);
  assert.equal(source.mainWeaponShutters.length, newMainWeaponShutters.length);
  assert.equal(source.subWeaponShutters.length, newSubWeaponShutters.length);
  assert.equal(source.chests.length, newChestItems.length);
  assert.equal(source.sealChests.length, newSealChestItems.length);
  assert.equal(source.shops.length, newShopItems.length);
  const mainWeaponShutters = shuffleSimply(newMainWeaponShutters, rng)
    .map((item, i) => ({ item, spot: source.mainWeaponShutters[i].spot }));
  const subWeaponShutters = shuffleSimply(newSubWeaponShutters, rng)
    .map((item, i) => ({ item, spot: source.subWeaponShutters[i].spot }));
  const chests = shuffleSimply(newChestItems, rng)
    .map((item, i) => ({ item, spot: source.chests[i].spot }));
  const sealChests = shuffleSimply(newSealChestItems, rng)
    .map((item, i) => ({ item, spot: source.sealChests[i].spot }));
  const shops = shuffleSimply(newShopItems, rng)
    .map((items, i) => ({ items, spot: source.shops[i].spot }));
  assert(shops.every(x => x.spot.talkNumber != null));
  return new Storage(mainWeaponShutters, subWeaponShutters, chests, sealChests, shops);
}

function distributeItems(items: ReadonlyArray<Item>, source: Storage, rng: prng) {
  assert.equal(
    items.length,
    source.mainWeaponShutters.length
    + source.subWeaponShutters.length
    + source.chests.length
    + source.sealChests.length
    + source.shops.length * 3,
  );
  const newMainWeaponShutters: Item[] = [];
  const newSubWeaponShutters: Item[] = [];
  const newChestItems: Item[] = [];
  const newSealChestItems: Item[] = [];
  const newShopItems: Item[] = [];
  const sorted = [...items].sort((a, b) => (
    Number(a.canDisplayInShop()) - Number(b.canDisplayInShop())
  ));
  sorted.forEach((item) => {
    switch (selectRandom(
      [
        source.mainWeaponShutters.length - newMainWeaponShutters.length,
        source.subWeaponShutters.length - newSubWeaponShutters.length,
        source.chests.length - newChestItems.length,
        source.sealChests.length - newSealChestItems.length,
        !item.canDisplayInShop() ? 0 : source.shops.length * 3 - newShopItems.length,
      ],
      rng,
    )) {
      case 0:
        newMainWeaponShutters.push(item);
        break;
      case 1:
        newSubWeaponShutters.push(item);
        break;
      case 2:
        newChestItems.push(item);
        break;
      case 3:
        newSealChestItems.push(item);
        break;
      case 4:
        newShopItems.push(item);
        break;
      default:
        throw new Error();
    }
  });
  return {
    newMainWeaponShutters,
    newSubWeaponShutters,
    newChestItems,
    newSealChestItems,
    newShopItems: (
      newShopItems
        .reduce<{ tmp: Item[]; list: [Item, Item, Item][] }>(
          ({ tmp, list }, c) => {
            tmp.push(c);
            if (tmp.length === 3) {
              list.push(<[Item, Item, Item]>tmp);
              return { list, tmp: [] };
            }
            return { tmp, list };
          },
          { tmp: [], list: [] },
      )
        .list
    ),
  };
}

function assertUnique(storage: Storage) {
  const nameMap = new Map<string, { spot: Spot; item: Item }>();
  const flagMap = new Map<string, { spot: Spot; item: Item }>();

  storage.mainWeaponShutters
    .concat(storage.subWeaponShutters)
    .concat(storage.chests)
    .concat(storage.sealChests)
    .concat(
      storage.shops
        .map(x => x.items.map(item => ({ item, spot: x.spot })))
        .reduce<ReadonlyArray<{ spot: Spot; item: Item }>>((p, c) => p.concat(c), []),
  )
    .forEach((x) => {
      if (
        x.item.name !== 'weights'
        && x.item.name !== 'shurikenAmmo'
        && x.item.name !== 'toukenAmmo'
        && x.item.name !== 'spearAmmo'
        && x.item.name !== 'flareGunAmmo'
        && x.item.name !== 'bombAmmo'
        && x.item.name !== 'ammunition'
        && x.item.name !== 'shellHorn'
        && x.item.name !== 'finder'
      ) {
        const key = `${x.item.type}:${x.item.name}`;
        if (nameMap.has(key)) {
          console.error(nameMap.get(key), x);
          assert.fail();
        }
        nameMap.set(key, x);
      }

      if (x.item.flag !== 65279
        && x.item.flag !== 753
        && x.item.flag !== 754) {
        const key = `${x.item.flag}`;
        if (flagMap.has(key)) {
          console.error(flagMap.get(key), x);
          assert.fail();
        }
        flagMap.set(key, x);
      }
    });
}
