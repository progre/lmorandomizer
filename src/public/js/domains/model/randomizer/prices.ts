//import Item from '../dataset/Item'
import {
	SubWeaponNumber,
	subWeaponNumbers,
	EquipmentNumber,
	equipmentNumbers,
	RomNumber,
	romNumbers,
} from './items';

export function getNewPrice (
  stringItemType: 'mainWeapon' | 'subWeapon' | 'equipment' | 'rom' |
  'seal',
  itemNumber : Number,
) {
	  switch (stringItemType) {
		  case 'mainWeapon': return getMainWeaponPrice(itemNumber);
		  case 'subWeapon': return getSubWeaponPrice(itemNumber as SubWeaponNumber);
		  case 'equipment': return getEquipmentPrice(itemNumber as EquipmentNumber);
		  case 'rom': return getRomPrice(itemNumber as RomNumber);
		  case 'seal': return getSealPrice(itemNumber);
		  default: throw new Error();
	  }
}
  
function getMainWeaponPrice(
  itemNumber: Number,
) {
	throw new Error();
	return 0;
}

function getSubWeaponPrice(
  subNumber: SubWeaponNumber,
) {
	switch (subNumber) {
		case subWeaponNumbers.shuriken: return 20;
		case subWeaponNumbers.touken: return 30;
		case subWeaponNumbers.spear: return 40;
		case subWeaponNumbers.flareGun: return 70;
		case subWeaponNumbers.bomb: return 120;
		// case subWeaponNumbers.pistol: 
		case subWeaponNumbers.weights: return 20;
		case subWeaponNumbers.ankhJewel: return 120;
		case subWeaponNumbers.buckler: return 80;
		case subWeaponNumbers.handScanner: return 20;
		case subWeaponNumbers.silverShield: return 150;
		case subWeaponNumbers.angelShield: return 200;
		case subWeaponNumbers.ammunition: return 500;
		default:
			return 101;
	}
}
function getEquipmentPrice(
  equipNumber: EquipmentNumber,
) {
	
	switch (equipNumber) {
		case equipmentNumbers.msx: throw new Error(); // msx
		case equipmentNumbers.shellHorn: return 60;
		case equipmentNumbers.waterproofCase: return 80;
		case equipmentNumbers.heatproofCase: return 100;
		case equipmentNumbers.finder: return 80;
		case equipmentNumbers.lampOfTime: return 200;
		// case equipmentNumbers.protectiveClothes: return 300;
		// case equipmentNumbers.talisman: return 150;
		case equipmentNumbers.scriptures: return 350;
		case equipmentNumbers.gauntlet: return 100;
		// case equipmentNumbers.ring: return 100;
		// case equipmentNumbers.glove: return 150;
		// case equipmentNumbers.keyOfEternity: return 60;
		// case equipmentNumbers.twinStatue: return 60;
		// case equipmentNumbers.bronzeMirror: return 50;
		// case equipmentNumbers.boots: return 80;
		// case equipmentNumbers.feather: return 100;
		case equipmentNumbers.bracelet: return 150;
		case equipmentNumbers.dragonBone: return 200;
		// case equipmentNumbers.grappleClaw: return 80;
		// case equipmentNumbers.magatamaJewel: return 80;
		// case equipmentNumbers.crucifix: return 200;
		// case equipmentNumbers.bookOfTheDead: return 200;
		// case equipmentNumbers.perfume: return 300;
		// case equipmentNumbers.ocarina: return 30;
		case equipmentNumbers.anchor: return 5;
		// case equipmentNumbers.womanStatue: return 110;
		// case equipmentNumbers.miniDoll: return 120;
		// case equipmentNumbers.eyeOfTruth: return 90;
		// case equipmentNumbers.serpentStaff: return 70;
		case equipmentNumbers.iceCape: return 200;
		case equipmentNumbers.helmet: return 90;
		// case equipmentNumbers.scalesphere: return 250;
		// case equipmentNumbers.crystalSkull: return 200;
		// case equipmentNumbers.djedPillar: return 100;
		// case equipmentNumbers.planeModel: return 160;
		// case equipmentNumbers.cogOfTheSoul: return 200;
		// case equipmentNumbers.pochetteKey: return ;
		// case equipmentNumbers.vessel: return 
		case equipmentNumbers.msx2: return 150;
		// case equipmentNumbers.diary: return 
		// case equipmentNumbers.mulanaTalisman: return 
		case equipmentNumbers.lampOfTimeSpecified: throw new Error();
		// case equipmentNumbers.maternityStatue: return 
		case equipmentNumbers.fakeHandScanner: return 150;
		// case equipmentNumbers.pepper: return 
		case equipmentNumbers.treasures: return 500;
		
		case equipmentNumbers.medicineOfLifeYellow: 
		case equipmentNumbers.medicineOfLifeGreen: 
		case equipmentNumbers.medicineOfLifeRed: throw new Error();
		
		case equipmentNumbers.fakeSilverShield: return 150;
		
		case equipmentNumbers.theTreasuresOfLaMurana: 
		case equipmentNumbers.sacredOrb:
		case equipmentNumbers.map: throw new Error()
		
		case equipmentNumbers.originSeal:
		case equipmentNumbers.birthSeal:
		case equipmentNumbers.lifeSeal:
		case equipmentNumbers.deathSeal: throw new Error();
		
		case equipmentNumbers.sweetClothing: throw new Error();
		
		
		
		default: return 102;
	}
}
function getRomPrice(
  romNumber: RomNumber,
)  {
	switch(romNumber) {
		case romNumbers.gameMaster: return 10;
		// case romNumbers.gameMaster2:
		case romNumbers.glyphReader: return 100;
		case romNumbers.ruinsRam8k: return 30;
		// ...
		case romNumbers.knightmare: return 200;
		// ...
		case romNumbers.castlevania: return 130;
		// ...
		case romNumbers.f1Spirit: return 180;
		// ...
		case romNumbers.sealOfElGiza: return 100;
		// ...
		case romNumbers.mahjongWizard: return 50;
		// ...
		case romNumbers.divinerSensation: return 80;
		// case romNumbers.snatcher: 
		case romNumbers.f1Spirit3d: return 5;
		default: return 103
	}
	return 103;
}

function getSealPrice(
  itemNumber: Number,
) {
	return 122;
}