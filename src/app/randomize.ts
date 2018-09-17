import Supplements from '../model/dataset/Supplements';
import { equipmentNumbers, subWeaponNumbers } from '../model/randomizer/items';
import randomizeItems from '../model/randomizer/randomizeItems';
import Script from '../util/scriptdat/Script';

export default async function randomize(
  script: Script,
  supplements: Supplements,
  config: {
    seed: string;
    easyMode: boolean;
  },
) {
  await randomizeItems(script, supplements, config.seed);
  if (config.easyMode) {
    script.addStartingItems(
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
