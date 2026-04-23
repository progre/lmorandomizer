mod debugger;
mod dll_injection;
mod find_func_addr;
mod utils;

use std::ffi::{OsStr, c_void};
use std::io::Write;
use std::mem::{forget, size_of};
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, iter};

use anyhow::Result;
use lmorandomizer_shared::Command;
use lmorandomizer_shared::ipc::IpcStream;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE, VirtualAllocEx, VirtualFreeEx,
};
use windows::Win32::System::Threading::{
    CREATE_SUSPENDED, CreateProcessW, DEBUG_ONLY_THIS_PROCESS, PROCESS_INFORMATION, ResumeThread,
    STARTUPINFOW, Wow64SuspendThread,
};
use windows::core::{Owned, PCWSTR};

use crate::launcher::debugger::{break_to_entry, continue_and_detatch};
use crate::launcher::dll_injection::do_dll_injection;

struct VirtualAllocatedMem<'a> {
    process: &'a Owned<HANDLE>,
    pub addr: *mut c_void,
}

impl<'a> VirtualAllocatedMem<'a> {
    pub fn new(process: &'a Owned<HANDLE>, size: usize) -> Self {
        Self {
            process,
            addr: unsafe { VirtualAllocEx(**process, None, size, MEM_COMMIT, PAGE_READWRITE) },
        }
    }
}

impl Drop for VirtualAllocatedMem<'_> {
    fn drop(&mut self) {
        unsafe { VirtualFreeEx(**self.process, self.addr, 0, MEM_RELEASE) }.unwrap();
    }
}

pub fn launch(exe_dir_path: &Path, exe_file_name: &str, custom_path: PathBuf) -> Result<()> {
    let current_exe = env::current_exe().unwrap();
    let dll_path = current_exe.parent().unwrap().join("hook.dll");
    let process_id = launch_and_inject(exe_dir_path, exe_file_name, &dll_path)?;

    let pipe_name = format!("lmorandomizer_for_{}", process_id);
    let mut stream = connect(&pipe_name)?;

    let cmd = Command::Init(custom_path);
    println!("{cmd:?}");
    stream.write_all(&rmp_serde::to_vec(&cmd).unwrap())?;

    Ok(())
}

fn launch_and_inject(exe_dir_path: &Path, exe_file_name: &str, dll_path: &Path) -> Result<u32> {
    let (process, _process_id, thread) =
        create_process_with_suspend_debug(exe_dir_path, exe_file_name)?;

    let (process_id, thread_id) = break_to_entry(*process, *thread)?;
    unsafe { Wow64SuspendThread(*thread) };
    continue_and_detatch(process_id, thread_id)?;

    do_dll_injection(process_id, dll_path)?;

    unsafe { ResumeThread(*thread) };

    forget(process);
    forget(thread);
    Ok(process_id)
}

fn create_process_with_suspend_debug(
    exe_dir_path: &Path,
    exe_file_name: &str,
) -> windows::core::Result<(Owned<HANDLE>, u32, Owned<HANDLE>)> {
    let cwd = to_wide_null(exe_dir_path.as_os_str());
    let exe_path = to_wide_null(exe_dir_path.join(exe_file_name).as_os_str());

    let si = STARTUPINFOW {
        cb: size_of::<STARTUPINFOW>() as u32,
        ..STARTUPINFOW::default()
    };
    let mut pi = PROCESS_INFORMATION::default();
    unsafe {
        CreateProcessW(
            PCWSTR(exe_path.as_ptr()),
            None,
            None,
            None,
            false,
            CREATE_SUSPENDED | DEBUG_ONLY_THIS_PROCESS,
            None,
            PCWSTR(cwd.as_ptr()),
            &si,
            &mut pi,
        )
    }?;
    let process = unsafe { Owned::new(pi.hProcess) };
    let thread = unsafe { Owned::new(pi.hThread) };

    Ok((process, pi.dwThreadId, thread))
}

fn to_wide_null(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(iter::once(0)).collect()
}

fn connect(pipe_name: &str) -> Result<IpcStream> {
    let mut i = 0;
    let stream = loop {
        match IpcStream::connect(pipe_name) {
            Ok(stream) => {
                break stream;
            }
            Err(err) => {
                if i < 50 {
                    std::thread::sleep(Duration::from_millis(500));
                    i += 1;
                    continue;
                }
                return Err(err.into());
            }
        }
    };

    Ok(stream)
}
