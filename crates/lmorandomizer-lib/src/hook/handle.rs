use std::ffi::c_char;

use addr_map_macro::addr_map;
use lmorandomizer_shared::lmo::Flags;
use windows::{
    Win32::{
        Foundation::HANDLE,
        Security::SECURITY_ATTRIBUTES,
        Storage::FileSystem::{
            FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE,
        },
    },
    core::PCSTR,
};

#[addr_map("assets/addr_map.toml", extern "cdecl")]
pub struct LmoHandle;

pub type CreateFileAFn = extern "system" fn(
    lpfilename: PCSTR,
    dwdesiredaccess: u32,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const SECURITY_ATTRIBUTES,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
    htemplatefile: HANDLE,
) -> HANDLE;
