use windows::{
    Win32::{
        Foundation::{
            DBG_CONTINUE, DBG_EXCEPTION_NOT_HANDLED, EXCEPTION_BREAKPOINT, HANDLE,
            INVALID_HANDLE_VALUE, STATUS_WX86_BREAKPOINT,
        },
        System::{
            Diagnostics::Debug::{
                ContinueDebugEvent, DEBUG_EVENT, DebugActiveProcessStop, EXCEPTION_DEBUG_EVENT,
                ReadProcessMemory, WOW64_CONTEXT, WOW64_CONTEXT_CONTROL, WaitForDebugEvent,
                Wow64GetThreadContext, Wow64SetThreadContext, WriteProcessMemory,
            },
            Threading::{INFINITE, ResumeThread},
        },
    },
    core::Result,
};

use crate::launcher::utils::image_base_addr;

pub fn break_to_entry(process: HANDLE, thread: HANDLE) -> windows::core::Result<(u32, u32)> {
    unsafe {
        let image_base = image_base_addr(process)?;
        let entry_rva = entry_rva(process, image_base)?;
        let entry = image_base + entry_rva;

        let mut original: u8 = 0;
        const INT3: u8 = 0xcc;
        ReadProcessMemory(process, entry as _, &mut original as *mut _ as _, 1, None)?;
        WriteProcessMemory(process, entry as _, &INT3 as *const _ as _, 1, None)?;

        if ResumeThread(thread) == INVALID_HANDLE_VALUE.0 as u32 {
            return Err(windows::core::Error::from_thread());
        }

        let (process_id, thread_id) = wait_breakpoint(entry)?;

        // 元バイト復元
        WriteProcessMemory(process, entry as _, &original as *const _ as _, 1, None)?;
        // EIP を戻す
        let mut ctx = WOW64_CONTEXT {
            ContextFlags: WOW64_CONTEXT_CONTROL,
            ..WOW64_CONTEXT::default()
        };
        Wow64GetThreadContext(thread, &mut ctx)?;
        ctx.Eip -= 1;
        Wow64SetThreadContext(thread, &ctx)?;

        Ok((process_id, thread_id))
    }
}

pub fn continue_and_detatch(process_id: u32, thread_id: u32) -> windows::core::Result<()> {
    unsafe {
        ContinueDebugEvent(process_id, thread_id, DBG_CONTINUE)?;
        DebugActiveProcessStop(process_id)?;
    }
    Ok(())
}

unsafe fn entry_rva(process: HANDLE, base: usize) -> windows::core::Result<usize> {
    let mut dos = [0u8; 0x40];
    unsafe { ReadProcessMemory(process, base as _, dos.as_mut_ptr() as _, dos.len(), None) }?;

    let e_lfanew = unsafe { *(dos.as_ptr().add(0x3C) as *const u32) } as usize;

    let addr = (base + e_lfanew) as _;
    let mut nt = [0u8; 0x200];
    unsafe { ReadProcessMemory(process, addr, nt.as_mut_ptr() as _, nt.len(), None) }?;

    // OptionalHeader.AddressOfEntryPoint (offset 0x28)
    Ok(unsafe { *(nt.as_ptr().add(0x28) as *const u32) } as usize)
}

fn wait_breakpoint(bp_addr: usize) -> Result<(u32, u32)> {
    let mut debug_event = DEBUG_EVENT::default();
    loop {
        unsafe {
            WaitForDebugEvent(&mut debug_event, INFINITE)?;
            let process_id = debug_event.dwProcessId;
            let thread_id = debug_event.dwThreadId;
            if debug_event.dwDebugEventCode == EXCEPTION_DEBUG_EVENT {
                let ex = debug_event.u.Exception;
                if matches!(
                    ex.ExceptionRecord.ExceptionCode,
                    EXCEPTION_BREAKPOINT | STATUS_WX86_BREAKPOINT
                ) {
                    let addr = ex.ExceptionRecord.ExceptionAddress as usize;
                    if addr == bp_addr {
                        break;
                    }
                    ContinueDebugEvent(process_id, thread_id, DBG_CONTINUE)?;
                    continue;
                }
            }
            ContinueDebugEvent(process_id, thread_id, DBG_EXCEPTION_NOT_HANDLED)?;
        }
    }
    Ok((debug_event.dwProcessId, debug_event.dwThreadId))
}
