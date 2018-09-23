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
  console.time('readScriptDat');
  const script = readScriptDat(scriptDat);
  console.timeEnd('readScriptDat');
  console.time('readSupplements');
  const supplements = new Supplements(supplementFiles);
  console.timeEnd('readSupplements');
  console.time('randomize');
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
