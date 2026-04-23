use windows::{
    Wdk::System::Threading::{NtQueryInformationProcess, ProcessBasicInformation},
    Win32::{
        Foundation::HANDLE,
        System::{Diagnostics::Debug::ReadProcessMemory, Threading::PROCESS_BASIC_INFORMATION},
    },
};

pub fn image_base_addr(process: HANDLE) -> windows::core::Result<usize> {
    let class = ProcessBasicInformation;
    let mut pbi = PROCESS_BASIC_INFORMATION::default();
    let p_pbi = &mut pbi as *mut _ as *mut _;
    let pbi_len = size_of::<PROCESS_BASIC_INFORMATION>() as u32;
    let mut ret_len = 0u32;
    unsafe { NtQueryInformationProcess(process, class, p_pbi, pbi_len, &mut ret_len) }.ok()?;

    let peb_base = pbi.PebBaseAddress as usize;
    // PEB レイアウト上の ImageBaseAddress オフセット:
    //   64bit: +0x10 (flags[4] + padding[4] + Mutant[8])
    //   32bit: +0x08 (flags[4] + Mutant[4])
    // どちらも 2 * size_of::<usize>() で求められる。
    let image_base_offset = 2 * size_of::<usize>();
    let addr = (peb_base + image_base_offset) as *const _;
    let mut image_base: usize = 0;
    let p_buf = &mut image_base as *mut _ as *mut _;
    unsafe { ReadProcessMemory(process, addr, p_buf, size_of::<usize>(), None) }?;

    Ok(image_base)
}
