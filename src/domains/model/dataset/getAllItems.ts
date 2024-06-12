import assert from '../../../assert';
import Script from '../../util/scriptdat/data/Script';
import { ShopItemData } from '../../util/scriptdat/format/ShopItemsData';
import Item from './Item';
import Supplements from './Supplements';

export default async function getAllItems(script: Script, supplements: Supplements) {
  return {
    mainWeapons: await mainWeapons(script, supplements),
    subWeapons: await subWeapons(script, supplements),
    chests: await chests(script, supplements),
    seals: await seals(script, supplements),
    shops: await shops(script, supplements),
  };
}

async function mainWeapons(script: Script, supplements: Supplements) {
  const mainWeaponsDataList = await script.mainWeapons();
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

async function subWeapons(script: Script, supplements: Supplements) {
  const subWeaponsDataList = await script.subWeapons();
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

async function chests(script: Script, supplements: Supplements) {
  const chestDataList = await script.chests();
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

async function seals(script: Script, supplements: Supplements) {
  const sealDataList = await script.seals();
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

async function shops(script: Script, supplements: Supplements) {
  const shopDataList = await script.shops();
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
