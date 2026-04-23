mod handle;
mod hook_manager;

use std::{
    fmt::Debug,
    mem::transmute,
    sync::{Arc, OnceLock},
};

use crate::hook_utils::hook_addr;
use hook_manager::LmoHookManager;
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

pub use handle::{CreateFileAFn, LmoHandle};

static LMO_HOOK_MANAGER: OnceLock<LmoHookManager> = OnceLock::new();

pub trait LmoDelegate: Debug {
    #[allow(clippy::too_many_arguments)]
    fn create_file_a(
        &self,
        lpfilename: PCSTR,
        dwdesiredaccess: u32,
        dwsharemode: FILE_SHARE_MODE,
        lpsecurityattributes: *const SECURITY_ATTRIBUTES,
        dwcreationdisposition: FILE_CREATION_DISPOSITION,
        dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
        htemplatefile: HANDLE,
        original: CreateFileAFn,
    ) -> HANDLE;
}

#[derive(Debug)]
pub struct OriginalAddrs {
    create_file_a: CreateFileAFn,
}

pub fn install_hook(
    handle: &LmoHandle,
    delegate: Arc<dyn LmoDelegate>,
) -> windows::core::Result<()> {
    let addrs = OriginalAddrs {
        create_file_a: {
            let original = hook_addr(handle.create_file_a.cast(), create_file_a_hook as _)?;
            unsafe { transmute::<*const (), CreateFileAFn>(original) }
        },
    };

    let mng = LmoHookManager::new(delegate, addrs);
    LMO_HOOK_MANAGER.set(mng).unwrap();
    Ok(())
}

extern "system" fn create_file_a_hook(
    lpfilename: PCSTR,
    dwdesiredaccess: u32,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const SECURITY_ATTRIBUTES,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
    htemplatefile: HANDLE,
) -> HANDLE {
    let mng = LMO_HOOK_MANAGER.get().unwrap();
    mng.delegate.create_file_a(
        lpfilename,
        dwdesiredaccess,
        dwsharemode,
        lpsecurityattributes,
        dwcreationdisposition,
        dwflagsandattributes,
        htemplatefile,
        mng.original_addrs.create_file_a,
    )
}
