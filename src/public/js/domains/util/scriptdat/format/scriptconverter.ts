import sha3 from 'js-sha3';
import Script from '../data/Script';

// tslint:disable:max-line-length
const SCRIPT_DAT_HASH = 'd18f3a643bee62db6870b35b1a1781bcc4067bd7409fa620168e16054ddc7ce645463b59e06d0768d87eff9ad9bdc1f0efd04dbc498d2e5de73d5a863a692a90';
const SCRIPT_DAT_EN_HASH = '146e1b6e9e63ed22fb84b3c38f4d25a0723b07fe3fefe9395af68d6eeaa3b1108b288847ec50114efff4e7600afccc68a983d681b94cbb55a507b21f45d52db7';
// tslint:enable:max-line-length

export function readScriptDat(wasm: any, file: ArrayBuffer) {
  if (!isValidScriptDat(file)) {
    throw new Error('invalidfile');
  }
  const txt = decode(wasm, file);
  return Script.parse(txt);
}

function decode(wasm: any, file: ArrayBuffer) {
  let str: string = <any>null;
  useNativeMemory(wasm, file.byteLength * 2, (toUtf16Ptr) => {
    let buffer: ArrayBuffer = <any>null;
    useNativeMemory(wasm, file.byteLength, (fromAsciiPtr) => {
      buffer = wasm.memory.buffer;
      const heap = new Uint8Array(buffer);
      heap.set(new Uint8Array(file), fromAsciiPtr);
      wasm.decode(file.byteLength, fromAsciiPtr, toUtf16Ptr);
    });
    const to = new Uint16Array(
      buffer.slice(toUtf16Ptr, toUtf16Ptr + file.byteLength * 2),
    );
    str = fromUtf16(to);
  });
  return str;
}

export function buildScriptDat(wasm: any, script: Script) {
  const str = script.stringify();
  return encode(wasm, str);
}

function encode(wasm: any, str: string) {
  const result = new Uint8Array(str.length);
  useNativeStringBuilder(wasm, str, (fromStringBuilderPtr) => {
    useNativeMemory(wasm, str.length, (toAsciiPtr) => {
      wasm.encode(fromStringBuilderPtr, str.length, toAsciiPtr);
      const buffer: ArrayBuffer = wasm.memory.buffer;
      result.set(new Uint8Array(buffer.slice(toAsciiPtr, toAsciiPtr + str.length)), 0);
    });
  });
  return result;
}

function useNativeStringBuilder(wasm: any, str: string, callback: (ptr: number) => void) {
  const ptr: number = wasm.create_string_builder_with_capacity(str.length);
  for (const char of str) {
    wasm.string_builder_append_unchecked(ptr, char.codePointAt(0));
  }
  callback(ptr);
  wasm.destroy_string_builder(ptr);
}

function useNativeMemory(wasm: any, size: number, callback: (ptr: number) => void) {
  const ptr: number = wasm.alloc(size);
  callback(ptr);
  wasm.free(ptr);
}

function fromUtf16(buffer: Uint16Array) {
  let str = '';
  for (let i = 0; i < buffer.length; i += 1024) {
    str += String.fromCodePoint.apply(null, buffer.slice(i, i + 1024));
  }
  return str;
}

export function isValidScriptDat(file: ArrayBuffer) {
  const fileHash = sha3.sha3_512(file);
  return fileHash === SCRIPT_DAT_HASH || fileHash === SCRIPT_DAT_EN_HASH;
}
