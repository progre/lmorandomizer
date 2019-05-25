import assert from 'assert';
// tslint:disable-next-line:no-implicit-dependencies
import electron from 'electron';
const { app } = electron.remote;
import fs from 'fs';
import util from 'util';
import { isValidScriptDat } from '../domains/util/scriptdat/format/scriptconverter';
import ScriptDatRepo from '../infrastructures/ScriptDatRepo';
import SupplementFileRepo from '../infrastructures/SupplementFileRepo';

const unlink = util.promisify(fs.unlink);

export default class App {
  private scriptDatRepo = new ScriptDatRepo();
  private supplementFileRepo = new SupplementFileRepo();
  private worker = new Worker('js/randomizer.js');

  async apply(
    installDirectory: string,
    options: {
      seed: string;
      easyMode: boolean;
      tabletSave: boolean;
      grailStart: boolean;
      scannerStart: boolean;
      gameMasterStart: boolean;
      readerStart: boolean;
      autoRegistration: boolean;
    },
  ) {
    const targetFilePath = `${installDirectory}/data/script.dat`;
    const oldBackupFilePath = `${app.getPath('userData')}/script.dat.bak`;
    const backupFilePath = `${installDirectory}/data/script.dat.bak`;

    let working;
    working = await this.readValidFileOrNull(backupFilePath);
    if (working == null) {
      working = await this.scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
      if (working == null) {
        return 'Unable to find La-Mulana install directory.';
      }
      if (!isValidScriptDat(working)) {
        working = await this.readValidFileOrNull(oldBackupFilePath);
        if (working == null) {
          await unlinkOldBackup();
          return 'Valid script is not found. Please re-install La-Mulana.';
        }
      }
      await this.writeValidScriptDat(backupFilePath, working);
    }
    const supplementFiles = await this.supplementFileRepo.read();

    const randomized
      = await this.randomizeOnWorker(supplementFiles, options, working);

    await this.scriptDatRepo.writeScriptDat(targetFilePath, randomized);
    await unlinkOldBackup();
    return 'Succeeded.';
  }

  async restore(installDirectory: string) {
    const targetFilePath = `${installDirectory}/data/script.dat`;
    const oldBackupFilePath = `${app.getPath('userData')}/script.dat.bak`;
    const backupFilePath = `${installDirectory}/data/script.dat.bak`;

    const targetFile = await this.scriptDatRepo.readFileOrNullIfNoEntry(targetFilePath);
    if (targetFile != null && isValidScriptDat(targetFile)) {
      await unlinkOldBackup();
      return 'Already clean.';
    }
    let working;
    working = await this.readValidFileOrNull(backupFilePath);
    if (working == null) {
      working = await this.readValidFileOrNull(oldBackupFilePath);
      if (working == null) {
        await unlinkOldBackup();
        return 'Backup is broken. Please re-install La-Mulana.';
      }
    }
    await this.writeValidScriptDat(targetFilePath, working);
    await unlinkOldBackup();
    return 'Succeeded.';
  }

  private async readValidFileOrNull(path: string) {
    const working = await this.scriptDatRepo.readFileOrNullIfNoEntry(path);
    if (working == null || !isValidScriptDat(working)) {
      return null;
    }
    return working;
  }

  private async randomizeOnWorker(
    supplementFiles: any,
    options: any,
    scriptDat: ArrayBuffer,
  ) {
    return new Promise<ArrayBuffer>((resolve, reject) => {
      this.worker.onmessage = (e) => {
        this.worker.onmessage = null;
        resolve(e.data);
      };
      this.worker.onerror = (err) => {
        this.worker.onerror = null;
        reject(err.error);
      };
      this.worker.postMessage(
        { supplementFiles, options, scriptDat },
        [scriptDat],
      );
    });
  }

  private async writeValidScriptDat(path: string, scriptDat: ArrayBuffer) {
    await this.scriptDatRepo.writeScriptDat(path, scriptDat);
    const outputed = await this.scriptDatRepo.readFileOrNullIfNoEntry(path);
    if (outputed == null || !isValidScriptDat(outputed)) {
      assert.fail();
    }
  }
}

async function unlinkOldBackup() {
  try {
    await unlink(`${app.getPath('userData')}/script.dat.bak`);
  } catch (err) {
    if (err.code === 'ENOENT') {
      return;
    }
    throw err;
  }
}
