import fs from 'fs';
import util from 'util';
import Script from '../domains/util/scriptdat/data/Script';
import { decode } from '../domains/util/scriptdat/format/codec';

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
    if (<any>1 === 0) {
      const txt = decode(buffer);
      fs.writeFileSync('tmp.txt', txt);
      fs.writeFileSync('tmp-shop.json', JSON.stringify(Script.parse(txt).shops()));
    }
  }
}
