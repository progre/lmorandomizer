import assert from '../assert';
import { isValidScriptDat } from '../domains/util/scriptdat/format/scriptconverter';
import ScriptDatRepo from '../infrastructures/ScriptDatRepo';
import SupplementFileRepo from '../infrastructures/SupplementFileRepo';
import randomize from './randomize';

export default class App {
  private scriptDatRepo = new ScriptDatRepo();
  private supplementFileRepo = new SupplementFileRepo();

  async apply(
    installDirectory: string,
    options: {
      seed: string;
      easyMode: boolean;
    },
  ) {
    const targetFilePath = `${installDirectory}/data/script.dat`;
    const backupFilePath = `${installDirectory}/data/script.dat.bak`;

    let working;
    working = await this.readValidFileOrNull(backupFilePath);
    if (working == null) {
      working = await this.scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
      if (working == null) {
        return 'Unable to find La-Mulana install directory.';
      }
      if (!isValidScriptDat(working)) {
        return 'Valid script is not found. Please re-install La-Mulana.';
      }
      await this.writeValidScriptDat(backupFilePath, working);
    }
    const supplementFiles = await this.supplementFileRepo.read();

    const randomized = await randomize(working, supplementFiles, options);

    await this.scriptDatRepo.writeScriptDat(targetFilePath, randomized);
    return 'Succeeded.';
  }

  async restore(installDirectory: string) {
    const targetFilePath = `${installDirectory}/data/script.dat`;
    const backupFilePath = `${installDirectory}/data/script.dat.bak`;

    const targetFile = await this.scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
    if (targetFile != null && isValidScriptDat(targetFile)) {
      return 'Already clean.';
    }
    let working;
    working = await this.readValidFileOrNull(backupFilePath);
    if (working == null) {
        return 'Backup is broken. Please re-install La-Mulana.';
    }
    await this.writeValidScriptDat(targetFilePath, working);
    return 'Succeeded.';
  }

  private async readValidFileOrNull(path: string) {
    const working = await this.scriptDatRepo.readFileOrNullIfNoEntry(path);
    if (working == null || !isValidScriptDat(working)) {
      return null;
    }
    return working;
  }

  private async writeValidScriptDat(path: string, scriptDat: ArrayBuffer) {
    await this.scriptDatRepo.writeScriptDat(path, scriptDat);
    const outputed = await this.scriptDatRepo.readFileOrNullIfNoEntry(path);
    if (outputed == null || !isValidScriptDat(outputed)) {
      assert.fail();
    }
  }
}
