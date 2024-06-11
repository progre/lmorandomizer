import { invoke } from "@tauri-apps/api/core";

export function isValidScriptDat(file: ArrayBuffer) {
  return invoke('is_valid_script_dat', { file: [...new Uint8Array(file)] });
}
