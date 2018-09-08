import fs from 'fs';
import util from 'util';
import { Settings } from '../types';

const readFile = util.promisify(fs.readFile);
const writeFile = util.promisify(fs.writeFile);

export default class SettingsRepo {
  constructor(
    private settingsFilePath: string,
  ) {
  }

  async get(): Promise<Settings> {
    try {
      return JSON.parse(
        await readFile(this.settingsFilePath, { encoding: 'utf8' }),
      );
    } catch (err) {
      console.error(err);
      return {
        // tslint:disable-next-line:insecure-random
        seed: Math.random().toString(36).slice(-8),
      };
    }
  }

  async set(settings: Settings) {
    await writeFile(this.settingsFilePath, JSON.stringify(settings));
  }
}
