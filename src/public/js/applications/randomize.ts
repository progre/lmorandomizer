import Supplements from '../domains/model/dataset/Supplements';
import randomizeItems from '../domains/model/randomizer/randomizeItems';
import {
  buildScriptDat,
  readScriptDat,
} from '../domains/util/scriptdat/format/scriptconverter';

export default function randomize(
  scriptDat: ArrayBuffer,
  supplementFiles: {
    weaponsYml: string;
    chestsYml: string;
    sealsYml: string;
    shopsYml: string;
    eventsYml: string;
  },
  options: {
    seed: string;
    easyMode: boolean;
  },
) {
  const script = readScriptDat(scriptDat);
  randomizeItems(script, new Supplements(supplementFiles), options.seed);
  if (options.easyMode) {
    script.addStartingItems(
      [
        // equipmentNumbers.holyGrail,
        100,
        // 102,
      ],
      [
        // subWeaponNumbers.handScanner,
      ],
    );
  }
  return buildScriptDat(script);
}
