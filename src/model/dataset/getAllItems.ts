import assert from 'assert';
import ScriptDat from '../../util/scriptdat/ScriptDat';
import { ShopItemData } from '../../util/scriptdat/ShopItemsData';
import Item from './Item';
import Supplements from './Supplements';

export default function getAllItems(scriptDat: ScriptDat, supplements: Supplements) {
  return {
    mainWeapons: mainWeapons(scriptDat, supplements),
    subWeapons: subWeapons(scriptDat, supplements),
    chests: chests(scriptDat, supplements),
    seals: seals(scriptDat, supplements),
    shops: shops(scriptDat, supplements),
  };
}

function mainWeapons(scriptDat: ScriptDat, supplements: Supplements) {
  const mainWeaponsDataList = scriptDat.mainWeapons();
  assert.equal(mainWeaponsDataList.length, supplements.mainWeapons.length);
  return supplements.mainWeapons.map((supplement, i) => {
    const data = mainWeaponsDataList[i];
    return new Item(
      supplement.name,
      'mainWeapon',
      data.mainWeaponNumber,
      1, // Count of main weapon is always 1.
      data.flag,
    );
  });
}

function subWeapons(scriptDat: ScriptDat, supplements: Supplements) {
  const subWeaponsDataList = scriptDat.subWeapons();
  assert.equal(
    subWeaponsDataList.length,
    supplements.subWeapons.length + Supplements.nightSurfaceSubWeaponCount,
  );
  return supplements.subWeapons.map((supplement, i) => {
    const data = subWeaponsDataList[i];
    return new Item(
      supplement.name,
      'subWeapon',
      data.subWeaponNumber,
      data.count,
      data.flag,
    );
  });
}

function chests(scriptDat: ScriptDat, supplements: Supplements) {
  const chestDataList = scriptDat.chests();
  assert.equal(
    chestDataList.length,
    supplements.chests.length + Supplements.nightSurfaceChestCount,
  );
  return supplements.chests.map((supplement, i) => {
    const data = chestDataList[i];
    return new Item(
      supplement.name,
      data.chestItemNumber < 100
        ? 'equipment'
        : 'rom',
      data.chestItemNumber < 100
        ? data.chestItemNumber
        : data.chestItemNumber - 100,
      1, // Count of chest item is always 1.
      data.flag,
    );
  });
}

function seals(scriptDat: ScriptDat, supplements: Supplements) {
  const sealDataList = scriptDat.seals();
  assert.equal(
    sealDataList.length,
    supplements.seals.length
    + Supplements.trueShrineOfTheMotherSealCount
    + Supplements.nightSurfacSealCount,
  );
  return supplements.seals.map((supplement, i) => {
    const data = sealDataList[i];
    return new Item(
      supplement.name,
      'seal',
      data.sealNumber,
      1, // Count of seal is always 1.
      data.flag,
    );
  });
}

function shops(scriptDat: ScriptDat, supplements: Supplements) {
  const shopDataList = scriptDat.shops();
  assert.equal(
    shopDataList.length,
    supplements.shops.length + Supplements.wareNoMiseCount,
  );
  return supplements.shops.map<[Item, Item, Item]>((supplement, i) => {
    const shop = shopDataList[i];
    const names = supplement.names.split(',').map(x => x.trim());
    assert.equal(names.length, 3);
    return [
      createItemFromShop(names[0], shop.items[0]),
      createItemFromShop(names[1], shop.items[1]),
      createItemFromShop(names[2], shop.items[2]),
    ];
  });
}

function createItemFromShop(name: string, data: ShopItemData) {
  return new Item(
    name,
    data.type === 0 ? 'subWeapon'
      : data.type === 1 ? 'equipment'
        : data.type === 2 ? 'rom'
          : (() => { throw new Error(); })(),
    data.number,
    data.count,
    data.flag,
  );
}
