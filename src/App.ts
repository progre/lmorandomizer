import assert from 'assert';
// tslint:disable-next-line:no-implicit-dependencies
import electron from 'electron';
const { app, BrowserWindow, ipcMain } = electron;
import fs from 'fs';
import seedrandom from 'seedrandom';
import util from 'util';
import randomize from './app/randomize';
import { initMacMenu } from './macmenu';
import SettingsRepo from './repo/SettingsRepo';
import SupplementsRepo from './repo/SupplementsRepo';
import { InitialParameters, Settings } from './types';
import ScriptDatRepo from './util/scriptdat/ScriptDatRepo';

const readFile = util.promisify(fs.readFile);

export default class App {
  static async create() {
    await new Promise((resolve, reject) => app.once('ready', resolve));
    initMacMenu();
    const settingsFilePath = `${app.getPath('userData')}/settings.json`;
    const settingsRepo = new SettingsRepo(settingsFilePath);
    const settings = await settingsRepo.get();
    const version = JSON.parse(
      await readFile(`${__dirname}/../package.json`, { encoding: 'utf8' }),
    ).version;
    return new this(version, settingsRepo, settings);
  }

  private win: electron.BrowserWindow;
  private scriptDatRepo = new ScriptDatRepo();

  constructor(
    version: string,
    private settingsRepo: SettingsRepo,
    private settings: Settings,
  ) {
    app.on('window-all-closed', app.quit.bind(app));
    this.win = new BrowserWindow({
      title: `La-Mulana Original Randomizer v${version}`,
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
    await new SupplementsRepo(`${__dirname}/res`).read(),
    {
      rng: seedrandom(seed),
    },
  );

  await scriptDatRepo.writeScriptDat(targetFilePath, scriptDat);
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
