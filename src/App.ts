import assert from 'assert';
// tslint:disable-next-line:no-implicit-dependencies
import electron from 'electron';
const { app, BrowserWindow, ipcMain } = electron;
import seedrandom from 'seedrandom';
import { initMacMenu } from './macmenu';
import { randomize } from './model/randomizer/randomize';
import ScriptDatRepo from './repo/ScriptDatRepo';
import SettingsRepo from './repo/SettingsRepo';
import { InitialParameters, Settings } from './types';

export default class App {
  static async create() {
    await new Promise((resolve, reject) => app.once('ready', resolve));
    initMacMenu();
    const settingsFilePath = `${app.getPath('userData')}/settings.json`;
    const settingsRepo = new SettingsRepo(settingsFilePath);
    const settings = await settingsRepo.get();
    return new this(settingsRepo, settings);
  }

  private win: electron.BrowserWindow;
  private scriptDatRepo = new ScriptDatRepo();

  constructor(
    private settingsRepo: SettingsRepo,
    private settings: Settings,
  ) {
    app.on('window-all-closed', app.quit.bind(app));
    this.win = new BrowserWindow({
      width: 800,
      height: 306,
      resizable: true,
      show: false,
    });
    this.win.on('ready-to-show', () => {
      this.win.show();
    });
    ipcMain.on('setSeed', async (ev: any, seed: string) => {
      try {
        this.settings.seed = seed;
        await this.settingsRepo.set(this.settings);
      } catch (err) {
        console.error(err);
      }
    });
    ipcMain.on('setInstallDirectory', async (ev: any, path: string) => {
      try {
        this.settings.installDirectory = path;
        await this.settingsRepo.set(this.settings);
      } catch (err) {
        console.error(err);
      }
    });
    ipcMain.on('apply', async (ev: any) => {
      try {
        const result = await apply(
          this.scriptDatRepo,
          `${this.settings.installDirectory}/data/script.dat`,
          `${app.getPath('userData')}/script.dat.bak`,
          this.settings.seed || '',
        );
        ev.sender.send('result', result);
      } catch (err) {
        console.error(err);
        ev.sender.send('result', err.toString());
      }
    });
    ipcMain.on('restore', async (ev: any) => {
      try {
        const result = await restore(
          this.scriptDatRepo,
          `${this.settings.installDirectory}/data/script.dat`,
          `${app.getPath('userData')}/script.dat.bak`,
        );
        ev.sender.send('result', result);
      } catch (err) {
        console.error(err);
        ev.sender.send('result', err.toString());
      }
    });
    const initialParams: InitialParameters = {
      seed: this.settings.seed || '',
      installDirectory: this.settings.installDirectory || '',
    };
    const search = encodeURIComponent(JSON.stringify(initialParams));
    this.win.loadURL(
      `file://${__dirname}/public/index.html?${search}`,
    );
  }
}

async function apply(
  scriptDatRepo: ScriptDatRepo,
  targetFilePath: string,
  workingFilePath: string,
  seed: string,
) {
  let { data: workingFile } = await scriptDatRepo.readValidScriptDat(workingFilePath);
  if (workingFile == null) {
    const { error, data: srcFile } = await scriptDatRepo.readValidScriptDat(targetFilePath);
    if (error != null) {
      return {
        noentry: 'Unable to find La-Mulana install directory.',
        invalidfile: 'Valid script is not found. Please re-install La-Mulana.',
      }[error.reason];
    }
    workingFile = srcFile!;
    await scriptDatRepo.writeScriptDat(workingFilePath, workingFile);
    if ((await scriptDatRepo.isValidScriptDat(workingFilePath)) == null) {
      assert.fail();
    }
  }

  const modifiedFile = await randomize(
    workingFile,
    {
      rng: seedrandom(seed),
    },
  );

  await scriptDatRepo.writeScriptDat(targetFilePath, <ArrayBuffer>modifiedFile.buffer);
  return 'Succeeded.';
}

async function restore(
  scriptDatRepo: ScriptDatRepo,
  targetFilePath: string,
  workingFilePath: string,
) {
  if (await scriptDatRepo.isValidScriptDat(targetFilePath)) {
    return 'Already clean.';
  }
  const { data: workingFile } = await scriptDatRepo.readValidScriptDat(workingFilePath);
  if (workingFile == null) {
    return 'Backup is broken. Please re-install La-Mulana.';
  }
  await scriptDatRepo.writeScriptDat(targetFilePath, workingFile);
  if ((await scriptDatRepo.isValidScriptDat(targetFilePath)) == null) {
    assert.fail();
  }
  return 'Succeeded.';
}
