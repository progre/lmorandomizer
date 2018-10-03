(<any>global).eval = global.eval = (arg) => {
  // seedrandom
  if (arg === 'this') {
    return global;
  }
  throw new Error(`Sorry, this app does not support window.eval().`);
};

import randomize from './applications/randomize';
declare const WebAssembly: any;
const wasmPromise = initWasm('../wasm/lib.wasm');

async function initWasm(path: string) {
  const res = await fetch(path);
  const bytes = await res.arrayBuffer();
  const { instance: { exports } } = await WebAssembly.instantiate(
    bytes,
    {
      env: {
        js_console_log(ptr: number, size: number) {
          const buffer = new Uint16Array(exports.memory.buffer, ptr, size / 2);
          const message = String.fromCodePoint(...buffer);
          console.log(message);
        },
        js_console_error(ptr: number, size: number) {
          const buffer = new Uint16Array(exports.memory.buffer, ptr, size / 2);
          const message = String.fromCodePoint(...buffer);
          console.error(message);
        },
      },
    },
  );
  exports.init();
  return exports;
}

onmessage = async (e) => {
  const wasm = await wasmPromise;
  const randomized = randomize(wasm, e.data.scriptDat, e.data.supplementFiles, e.data.options);
  postMessage(randomized.buffer, <any>undefined, [randomized.buffer]);
};
