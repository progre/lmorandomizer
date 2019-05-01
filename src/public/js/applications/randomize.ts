import Supplements from '../domains/model/dataset/Supplements';
import randomizeItems from '../domains/model/randomizer/randomizeItems';
import {
  buildScriptDat,
  readScriptDat,
} from '../domains/util/scriptdat/format/scriptconverter';
import {
  equipmentNumbers,
  subWeaponNumbers,
} from '../domains/model/randomizer/items'

export default function randomize(
  wasm: any,
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
	tabletSave: boolean;
  },
) {
  console.time('readScriptDat');
  const script = readScriptDat(wasm, scriptDat);
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
        equipmentNumbers.holyGrail,
        100,
        102,
      ],
      [
        subWeaponNumbers.handScanner,
      ],
    );
    console.timeEnd('addItems');
  }
  if (options.tabletSave) {
    console.time('tabletSave');
    script.tabletSave();
	console.timeEnd('tabletSave');
  }
  console.time('build');
  const output = buildScriptDat(wasm, script);
  console.timeEnd('build');
  return output;
}
