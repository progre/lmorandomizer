import Supplements from '../model/dataset/Supplements';
import { equipmentNumbers, subWeaponNumbers } from '../model/randomizer/items';
import randomizeItems from '../model/randomizer/randomizeItems';
import ScriptDat from '../util/scriptdat/ScriptDat';

export default async function randomize(
  scriptDat: ScriptDat,
  supplements: Supplements,
  config: {
    seed: string;
    easyMode: boolean;
  },
) {
  randomizeItems(scriptDat, supplements, config.seed);
  if (config.easyMode) {
    scriptDat.addStartingItems(
      [
        equipmentNumbers.holyGrail,
        100,
        102,
      ],
      [
        subWeaponNumbers.handScanner,
      ],
    );
  }
}
