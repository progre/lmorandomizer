import assert from 'assert';
import ScriptDat from '../../util/scriptdat/ScriptDat';
import getAllItems from './getAllItems';
import Item from './Item';
import Spot from './Spot';
import Storage from './Storage';
import Supplements from './Supplements';

export default async function createSource(scriptDat: ScriptDat, supplements: Supplements) {
  const allItems = getAllItems(scriptDat, supplements);
  const enumerateItems = (
    allItems.mainWeapons
      .concat(allItems.subWeapons)
      .concat(allItems.chests)
      .concat(allItems.seals)
      .concat(allItems.shops.reduce<Item[]>((p, c) => p.concat(c), []))
  );
  warnMissingRequirements(supplements, enumerateItems);
  const chestDataList = scriptDat.chests();
  assert.equal(
    chestDataList.length,
    supplements.chests.length + Supplements.nightSurfaceChestCount,
  );
  const shops = scriptDat.shops();
  return new Storage(
    allItems.mainWeapons.map((item, i) => {
      const supplement = supplements.mainWeapons[i];
      const spot = new Spot(
        'weaponShutter',
        parseRequirements(supplement.requirements || null, enumerateItems),
        null,
      );
      return { spot, item };
    }),
    allItems.subWeapons.map((item, i) => {
      const supplement = supplements.subWeapons[i];
      const spot = new Spot(
        'weaponShutter',
        parseRequirements(supplement.requirements || null, enumerateItems),
        null,
      );
      return { spot, item };
    }),
    allItems.chests.map((item, i) => {
      const supplement = supplements.chests[i];
      const spot = new Spot(
        'chest',
        parseRequirements(supplement.requirements || null, enumerateItems),
        null,
      );
      return { spot, item };
    }),
    allItems.seals.map((item, i) => {
      const supplement = supplements.seals[i];
      const spot = new Spot(
        'sealChest',
        parseRequirements(supplement.requirements || null, enumerateItems),
        null,
      );
      return { spot, item };
    }),
    allItems.shops.map((items, i) => {
      const supplement = supplements.shops[i];
      const shop = shops[i];
      const spot = new Spot(
        'shop',
        parseRequirements(supplement.requirements || null, enumerateItems),
        shop.talkNumber,
      );
      return { spot, items };
    }),
  );
}

function parseRequirements(
  requirements: ReadonlyArray<ReadonlyArray<string>> | null,
  allItems: ReadonlyArray<Item>,
) {
  if (requirements == null) {
    return null;
  }
  return requirements.map(y => (
    y.map(z => allItems.filter(w => w.name === z)[0])
      .filter(z => z != null)
  ));
}

function warnMissingRequirements(
  supplements: Supplements,
  allItems: ReadonlyArray<Item>,
) {
  const set = new Set();
  addSupplementTo(set, supplements.mainWeapons);
  addSupplementTo(set, supplements.subWeapons);
  addSupplementTo(set, supplements.chests);
  addSupplementTo(set, supplements.seals);
  addSupplementTo(set, supplements.shops);
  [...set]
    .filter(x => allItems.every(y => y.name !== x))
    .sort()
    .forEach((x) => {
      console.warn(`WARN: missing item: ${x}`);
    });
}

function addSupplementTo(
  set: Set<string>,
  supplement: ReadonlyArray<{ requirements?: ReadonlyArray<ReadonlyArray<string>> }>,
) {
  supplement.map(x => x.requirements || []).forEach((item) => {
    item.forEach((group) => {
      group.forEach((requirement) => {
        set.add(requirement);
      });
    });
  });
}
