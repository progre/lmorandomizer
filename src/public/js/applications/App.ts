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

export default class App {
  private scriptDatRepo = new ScriptDatRepo();
  private supplementFileRepo = new SupplementFileRepo();

  async apply(
    targetFilePath: string,
    workingFilePath: string,
    options: {
      seed: string;
      easyMode: boolean;
    },
  ) {
    let scriptDat = await this.scriptDatRepo.readFileOrNullIfNoEntry(workingFilePath);
    if (scriptDat == null || !isValidScriptDat(scriptDat)) {
      const srcFile = await this.scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
      if (srcFile == null) {
        return 'Unable to find La-Mulana install directory.';
      }
      if (!isValidScriptDat(srcFile)) {
        return 'Valid script is not found. Please re-install La-Mulana.';
      }
      scriptDat = srcFile;
      await this.scriptDatRepo.writeScriptDat(workingFilePath, scriptDat);
      const outputed = await this.scriptDatRepo.readFileOrNullIfNoEntry(workingFilePath);
      if (outputed == null || !isValidScriptDat(outputed)) {
        assert.fail();
      }
    }

    const supplementFiles = await this.supplementFileRepo.read();

    const script = await readScriptDat(scriptDat);
    await randomize(
      script,
      new Supplements(supplementFiles),
      options,
    );

    await this.scriptDatRepo.writeScriptDat(targetFilePath, await buildScriptDat(script));
    return 'Succeeded.';
  }

  async restore(
    targetFilePath: string,
    workingFilePath: string,
  ) {
    const targetFile = await this.scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
    if (targetFile != null && isValidScriptDat(targetFile)) {
      return 'Already clean.';
    }
    const working = await this.scriptDatRepo.readFileOrNullIfNoEntry(workingFilePath);
    if (working == null || !isValidScriptDat(working)) {
      return 'Backup is broken. Please re-install La-Mulana.';
    }
    await this.scriptDatRepo.writeScriptDat(targetFilePath, working);
    const outputed = await this.scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
    if (outputed == null || !isValidScriptDat(outputed)) {
      assert.fail();
    }
    return 'Succeeded.';
  }
}
