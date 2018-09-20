import assert from 'assert';
import randomize from '../domains/app/randomize';
import Supplements from '../domains/model/dataset/Supplements';
import {
  buildScriptDat,
  isValidScriptDat,
  readScriptDat,
} from '../domains/util/scriptdat/format/scriptconverter';
import ScriptDatRepo from '../infrastructures/ScriptDatRepo';
import SupplementFileRepo from '../infrastructures/SupplementFileRepo';

export async function apply(
  scriptDatRepo: ScriptDatRepo,
  targetFilePath: string,
  workingFilePath: string,
  options: {
    seed: string;
    easyMode: boolean;
  },
) {
  let scriptDat = await scriptDatRepo.readFileOrNullIfNoEntry(workingFilePath);
  if (scriptDat == null || !isValidScriptDat(scriptDat)) {
    const srcFile = await scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
    if (srcFile == null) {
      return 'Unable to find La-Mulana install directory.';
    }
    if (!isValidScriptDat(srcFile)) {
      return 'Valid script is not found. Please re-install La-Mulana.';
    }
    scriptDat = srcFile;
    await scriptDatRepo.writeScriptDat(workingFilePath, scriptDat);
    const outputed = await scriptDatRepo.readFileOrNullIfNoEntry(workingFilePath);
    if (outputed == null || !isValidScriptDat(outputed)) {
      assert.fail();
    }
  }

  const supplementFiles = await new SupplementFileRepo().read();

  const script = await readScriptDat(scriptDat);
  await randomize(
    script,
    new Supplements(supplementFiles),
    options,
  );

  await scriptDatRepo.writeScriptDat(targetFilePath, await buildScriptDat(script));
  return 'Succeeded.';
}

export async function restore(
  scriptDatRepo: ScriptDatRepo,
  targetFilePath: string,
  workingFilePath: string,
) {
  const targetFile = await scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
  if (targetFile != null && isValidScriptDat(targetFile)) {
    return 'Already clean.';
  }
  const working = await scriptDatRepo.readFileOrNullIfNoEntry(workingFilePath);
  if (working == null || !isValidScriptDat(working)) {
    return 'Backup is broken. Please re-install La-Mulana.';
  }
  await scriptDatRepo.writeScriptDat(targetFilePath, working);
  const outputed = await scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
  if (outputed == null || !isValidScriptDat(outputed)) {
    assert.fail();
  }
  return 'Succeeded.';
}
