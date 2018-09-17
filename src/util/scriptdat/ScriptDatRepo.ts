import fs from 'fs';
import sha3 from 'js-sha3';
import util from 'util';
import { decode, encode } from './codec';
import Script from './Script';

const readFile = util.promisify(fs.readFile);
const writeFile = util.promisify(fs.writeFile);

// tslint:disable:max-line-length
const SCRIPT_DAT_HASH = 'd18f3a643bee62db6870b35b1a1781bcc4067bd7409fa620168e16054ddc7ce645463b59e06d0768d87eff9ad9bdc1f0efd04dbc498d2e5de73d5a863a692a90';
const SCRIPT_DAT_EN_HASH = '146e1b6e9e63ed22fb84b3c38f4d25a0723b07fe3fefe9395af68d6eeaa3b1108b288847ec50114efff4e7600afccc68a983d681b94cbb55a507b21f45d52db7';
// tslint:enable:max-line-length

export default class ScriptDatRepo {
  async isValidScriptDat(path: string) {
    const { scriptDat } = await this.readValidScriptDat(path);
    return scriptDat != null;
  }

  async readValidScriptDat(path: string) {
    const data = await readFileOrNullIfNoEntry(path);
    if (data == null) {
      return { error: { reason: <'noentry'>'noentry' } };
    }
    if (!isValidScriptDatFile(data)) {
      return { error: { reason: <'invalidfile'>'invalidfile' } };
    }
    const txt = await decode(data);
    return { scriptDat: Script.parse(txt) };
  }

  async writeScriptDat(path: string, scriptDat: Script) {
    if (<any>1 === 1) {
      // await writeFile('./tmp.txt', scriptDat.stringify());
      // await writeFile('./tmp-shop.json', JSON.stringify(scriptDat.shops()));
    }
    const dat = await encode(scriptDat.stringify());
    await writeFile(path, Buffer.from(<ArrayBuffer>dat.buffer));
  }
}

async function readFileOrNullIfNoEntry(path: string) {
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

function isValidScriptDatFile(file: ArrayBuffer) {
  const fileHash = sha3.sha3_512(file);
  return fileHash === SCRIPT_DAT_HASH || fileHash === SCRIPT_DAT_EN_HASH;
}
