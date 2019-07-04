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
  
// whip 20
// chain whip 100
// mace 200
// knife 75
// axe 130
// keysword 50
// katana 145

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
		case subWeaponNumbers.shuriken: return 5;
		case subWeaponNumbers.touken: return 10;
		case subWeaponNumbers.spear: return 10;
		case subWeaponNumbers.flareGun: return 10;
		case subWeaponNumbers.bomb: return 40;
		case subWeaponNumbers.pistol: return 150;
		case subWeaponNumbers.weights: return 10;
		case subWeaponNumbers.ankhJewel: return 90;
		case subWeaponNumbers.buckler: return 80;
		case subWeaponNumbers.handScanner: return 50;
		case subWeaponNumbers.silverShield: return 150;
		case subWeaponNumbers.angelShield: return 200;
		case subWeaponNumbers.ammunition: return 300;
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
		case equipmentNumbers.lampOfTime: return 150;
		case equipmentNumbers.protectiveClothes: return 300;
		case equipmentNumbers.talisman: return 130;
		case equipmentNumbers.scriptures: return 350;
		case equipmentNumbers.gauntlet: return 110;
		case equipmentNumbers.ring: return 115;
		case equipmentNumbers.glove: return 55;
		case equipmentNumbers.keyOfEternity: return 40;
		case equipmentNumbers.twinStatue: return 40;
		case equipmentNumbers.bronzeMirror: return 30;
		case equipmentNumbers.boots: return 60;
		case equipmentNumbers.feather: return 75;
		case equipmentNumbers.bracelet: return 150;
		case equipmentNumbers.dragonBone: return 200;
		case equipmentNumbers.dragonBone: return 115;
		case equipmentNumbers.grappleClaw: return 60;
		case equipmentNumbers.magatamaJewel: return 130;
		case equipmentNumbers.crucifix: return 200;
		case equipmentNumbers.bookOfTheDead: return 135;
		case equipmentNumbers.perfume: return 300;
		case equipmentNumbers.ocarina: return 30;
		case equipmentNumbers.anchor: return 5;
		case equipmentNumbers.womanStatue: return 100;
		case equipmentNumbers.miniDoll: return 90;
		case equipmentNumbers.eyeOfTruth: return 190;
		case equipmentNumbers.serpentStaff: return 95;
		case equipmentNumbers.iceCape: return 100;
		case equipmentNumbers.helmet: return 90;
		case equipmentNumbers.scalesphere: return 105;
		case equipmentNumbers.crystalSkull: return 125;
		case equipmentNumbers.djedPillar: return 75;
		case equipmentNumbers.planeModel: return 115;
		case equipmentNumbers.cogOfTheSoul: return 120;
		case equipmentNumbers.pochetteKey: return 125;
		case equipmentNumbers.vessel: return 160;
		case equipmentNumbers.msx2: return 150;
		case equipmentNumbers.diary: return 115;
		case equipmentNumbers.mulanaTalisman: return 115;
		case equipmentNumbers.lampOfTimeSpecified: throw new Error();
		case equipmentNumbers.maternityStatue: throw new Error()
		case equipmentNumbers.fakeHandScanner: return 50;
		case equipmentNumbers.pepper: return 175;
		case equipmentNumbers.treasures: return 200;
		
		case equipmentNumbers.medicineOfLifeYellow: 
		case equipmentNumbers.medicineOfLifeGreen: 
		case equipmentNumbers.medicineOfLifeRed: throw new Error();
		
		case equipmentNumbers.fakeSilverShield: return 150;
		
		case equipmentNumbers.theTreasuresOfLaMurana: 
		case equipmentNumbers.sacredOrb: throw new Error() // make 90 if they can work in shops

		case equipmentNumbers.map: return 10;
		
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
		case romNumbers.gameMaster2: return 40;
		case romNumbers.glyphReader: return 30;
		case romNumbers.ruinsRam8k: return 30;
		case romNumbers.ruinsRam16k: return 60;
		case romNumbers.unreleasedRom: return 66;
		case romNumbers.pr3: return 66;
		case romNumbers.gr3: return 66;
		case romNumbers.athleticLand: return 117;
		case romNumbers.antarcticAdventure: return 57;
		case romNumbers.videoHustler: return 127;
		case romNumbers.comicBakery: return 57;
		case romNumbers.cabbagePatchKids: return 117;
		case romNumbers.hyperRally: return 22;
		case romNumbers.yieArKungFu: return 102;
		case romNumbers.roadFighter: return 22;
		case romNumbers.yieArKungFu2: return 102;
		case romNumbers.knightmare: return 17;
		case romNumbers.twinbee: return 52;
		case romNumbers.shinSynthesizer: return 12;
		case romNumbers.penguinAdventure: return 52;
		case romNumbers.castlevania: return 137;
		case romNumbers.kingKong2: return 147;
		case romNumbers.qbert: return 87;
		case romNumbers.firebird: return 147
		case romNumbers.mazeOfGalious: return 17;
		case romNumbers.metalGear: return 32;
		case romNumbers.gradius2: return 52;
		case romNumbers.f1Spirit: return 180; // can be default, rom does nothing
		case romNumbers.shalom: return 87;
		case romNumbers.breakShot: return 127;
		case romNumbers.salamander: return 52;
		case romNumbers.sealOfElGiza: return 17;
		case romNumbers.contra: return 5;
		case romNumbers.mahjongWizard: return 137;
		case romNumbers.metalGear2: return 32;
		case romNumbers.divinerSensation: return 87;
		case romNumbers.snatcher: return 12;
		case romNumbers.f1Spirit3d: return 5;
		case romNumbers.sdSnatcher: return 12;
		case romNumbers.badlands: return 22;
		case romNumbers.a1Spirit: return 22;
		default: return 103
	}
	return 103;
}

function getSealPrice(
  itemNumber: Number,
) {
	return 122;
}
