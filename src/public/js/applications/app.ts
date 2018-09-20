import assert from 'assert';
import randomize from '../domains/app/randomize';
import SupplementsRepo from '../domains/repo/SupplementsRepo';
import ScriptDatRepo from '../domains/util/scriptdat/ScriptDatRepo';

export async function apply(
  scriptDatRepo: ScriptDatRepo,
  targetFilePath: string,
  workingFilePath: string,
  dirName: string,
  options: {
    seed: string;
    easyMode: boolean;
  },
) {
  let { scriptDat } = await scriptDatRepo.readValidScriptDat(workingFilePath);
  if (scriptDat == null) {
    const { error, scriptDat: srcFile } = await scriptDatRepo.readValidScriptDat(targetFilePath);
    if (error != null) {
      return {
        noentry: 'Unable to find La-Mulana install directory.',
        invalidfile: 'Valid script is not found. Please re-install La-Mulana.',
      }[error.reason];
    }
    scriptDat = srcFile!;
    await scriptDatRepo.writeScriptDat(workingFilePath, scriptDat);
    if ((await scriptDatRepo.isValidScriptDat(workingFilePath)) == null) {
      assert.fail();
    }
  }

  await randomize(
    scriptDat,
    await new SupplementsRepo(`${dirName}/res`).read(),
    options,
  );

  await scriptDatRepo.writeScriptDat(targetFilePath, scriptDat);
  return 'Succeeded.';
}

export async function restore(
  scriptDatRepo: ScriptDatRepo,
  targetFilePath: string,
  workingFilePath: string,
) {
  if (await scriptDatRepo.isValidScriptDat(targetFilePath)) {
    return 'Already clean.';
  }
  const { scriptDat } = await scriptDatRepo.readValidScriptDat(workingFilePath);
  if (scriptDat == null) {
    return 'Backup is broken. Please re-install La-Mulana.';
  }
  await scriptDatRepo.writeScriptDat(targetFilePath, scriptDat);
  if ((await scriptDatRepo.isValidScriptDat(targetFilePath)) == null) {
    assert.fail();
  }
  return 'Succeeded.';
}
