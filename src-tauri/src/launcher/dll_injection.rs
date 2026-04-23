use std::{
    ffi::OsString,
    mem::{size_of, transmute},
    os::{raw::c_void, windows::ffi::OsStringExt},
    path::Path,
    ptr::null_mut,
};

use anyhow::{Result, bail};
use windows::{
    Win32::{
        Foundation::{GetLastError, HANDLE, HMODULE, MAX_PATH},
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            ProcessStatus::{EnumProcessModules, GetModuleFileNameExW},
            Threading::{
                CreateRemoteThread, LPTHREAD_START_ROUTINE, OpenProcess, PROCESS_ALL_ACCESS,
                WaitForSingleObject,
            },
        },
    },
    core::{HSTRING, Owned},
};

use crate::launcher::{VirtualAllocatedMem, find_func_addr::find_func_addr};

pub fn do_dll_injection(process_id: u32, dll_path: &Path) -> Result<bool> {
    if !dll_path.exists() {
        bail!("DLL not found.");
    }
    let process = unsafe {
        let process = OpenProcess(PROCESS_ALL_ACCESS, false, process_id)?;
        Owned::new(process)
    };

    if has_dll(*process, dll_path) {
        return Ok(false);
    }

    let dll_path_hstr = HSTRING::from(dll_path);
    let dll_path_hstr_range = dll_path_hstr.as_ptr_range();
    let dll_path_hstr_size = dll_path_hstr_range.end as usize - dll_path_hstr_range.start as usize;
    let remote_dll_path_wstr = VirtualAllocatedMem::new(&process, dll_path_hstr_size);

    let base_addr = remote_dll_path_wstr.addr;
    let buf = dll_path_hstr_range.start as _;
    unsafe { WriteProcessMemory(*process, base_addr, buf, dll_path_hstr_size, None) }.unwrap();
    let load_library_w_addr = find_func_addr(process_id, "KERNEL32.DLL", "LoadLibraryW").unwrap();

    let start_addr = unsafe { transmute::<usize, LPTHREAD_START_ROUTINE>(load_library_w_addr) };
    let param: Option<*const c_void> = Some(remote_dll_path_wstr.addr);
    let thread = unsafe {
        Owned::new(CreateRemoteThread(*process, None, 0, start_addr, param, 0, None).unwrap())
    };

    unsafe { WaitForSingleObject(*thread, u32::MAX) }; // wait thread

    Ok(true)
}

fn has_dll(process: HANDLE, dll_path: &Path) -> bool {
    enum_process_modules(process)
        .unwrap()
        .iter()
        .map(|module| get_module_file_name_ex(process, *module).unwrap())
        .any(|x| Path::new(&x) == dll_path)
}

fn enum_process_modules(process: HANDLE) -> windows::core::Result<Vec<HMODULE>> {
    let mut needed = 0u32;
    unsafe { EnumProcessModules(process, null_mut(), 0, &mut needed) }?;

    let count = needed as usize / size_of::<HMODULE>();
    let mut modules = vec![HMODULE::default(); count];
    unsafe { EnumProcessModules(process, modules.as_mut_ptr(), needed, &mut needed) }?;
    Ok(modules)
}

fn get_module_file_name_ex(process: HANDLE, module: HMODULE) -> windows::core::Result<OsString> {
    let mut size = MAX_PATH as usize;
    loop {
        let mut buf = vec![0u16; size];
        let len = unsafe { GetModuleFileNameExW(Some(process), Some(module), &mut buf) } as usize;
        if len == 0 {
            return Err(unsafe { GetLastError() }.into());
        }
        if len < size - 1 {
            return Ok(OsString::from_wide(&buf[..len]));
        }
        size *= 2;
    }
}
