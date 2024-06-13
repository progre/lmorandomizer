import { invoke } from '@tauri-apps/api/core';
import Supplements from '../domains/model/dataset/Supplements';
import { EquipmentNumber } from '../domains/model/randomizer/items';
import Script from '../domains/util/scriptdat/data/Script';

export default async function randomize(
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
  let script: Script = await invoke('read_script_dat', { file: [...new Uint8Array(scriptDat)] });
  console.timeEnd('readScriptDat');
  console.time('readSupplements');
  const supplements: Supplements = await invoke('create_supplements', { supplementFiles });
  console.timeEnd('readSupplements');
  console.time('randomize');
  script = await invoke('randomize_items', { script, supplements, seed: options.seed });
  script = Script.from_object(script);
  console.timeEnd('randomize');
  if (options.easyMode) {
    console.time('addItems');
    await script.addStartingItems(
      [
        // equipmentNumbers.holyGrail,
        <EquipmentNumber>100,
        // 102,
      ],
      [
        // subWeaponNumbers.handScanner,
      ],
    );
    console.timeEnd('addItems');
  }
  console.time('build');
  const output = Uint8Array.from(await invoke('build_script_dat', { script }));
  console.timeEnd('build');
  return output;
}