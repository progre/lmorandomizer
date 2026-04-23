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
