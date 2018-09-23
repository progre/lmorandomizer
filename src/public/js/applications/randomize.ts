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
  console.time('read');
  const script = readScriptDat(scriptDat);
  console.timeEnd('read');
  console.time('randomize');
  const supplements = new Supplements(supplementFiles);
  randomizeItems(script, supplements, options.seed);
  console.timeEnd('randomize');
  if (options.easyMode) {
    console.time('addItems');
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
    console.timeEnd('addItems');
  }
  console.time('build');
  const output = buildScriptDat(script);
  console.timeEnd('build');
  return output;
}
