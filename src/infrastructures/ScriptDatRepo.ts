import { invoke } from '@tauri-apps/api/core';
// import Script from '../domains/util/scriptdat/data/Script';
// import { decode } from '../domains/util/scriptdat/format/codec';

export default class ScriptDatRepo {
  async readFileOrNullIfNoEntry(path: string) {
    const buf: number[] | null = await invoke('read_file', { path });
    if (buf == null) {
      return null;
    }
    return Uint8Array.from(buf);
  }

  async writeScriptDat(path: string, buffer: ArrayBuffer) {
    await invoke('write_file', {
      path,
      contents: [...new Uint8Array(buffer)],
    });
    if (<any>1 === 0) {
      // const txt = decode(buffer);
      // fs.writeFileSync('tmp.txt', txt);
      // fs.writeFileSync('tmp-shop.json', JSON.stringify(Script.parse(txt).shops()));
    }
  }
}
