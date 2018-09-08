import { prng } from 'seedrandom';
import { Supplements } from '../model/dataset/types';
import { equipmentNumbers, subWeaponNumbers } from '../model/randomizer/items';
import randomizeItems from '../model/randomizer/randomizeItems';
import ScriptDat from '../util/scriptdat/ScriptDat';

export default async function randomize(
  scriptDat: ScriptDat,
  supplements: Supplements,
  config: { rng: prng },
) {
  randomizeItems(scriptDat, supplements, config.rng);
  if ((<any>1) === 1) {
    scriptDat.addStartingItems(
      [
        // ...`${items.sacredOrb},`.repeat(5).split(',').map(Number),
        // equipmentNumbers.feather,
        // equipmentNumbers.grappleClaw,
        // equipmentNumbers.boots,
        // equipmentNumbers.serpentStaff,
      ],
      [
        subWeaponNumbers.shuriken,
      ],
    );
  }
}
