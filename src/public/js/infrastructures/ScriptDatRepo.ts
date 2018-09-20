import fs from 'fs';
import util from 'util';

const readFile = util.promisify(fs.readFile);
const writeFile = util.promisify(fs.writeFile);

export default class ScriptDatRepo {
  async readFileOrNullIfNoEntry(path: string) {
    try {
      return <ArrayBuffer>(await readFile(path)).buffer;
    } catch (err) {
      if (err.code !== 'ENOENT') {
        throw err;
      }
      // not found
      return null;
    }
  }

  async writeScriptDat(path: string, buffer: ArrayBuffer) {
    await writeFile(path, Buffer.from(buffer));
  }
}
