use std::ptr::{read_unaligned, write_unaligned};

use windows::Win32::System::Memory::{
    PAGE_EXECUTE_WRITECOPY, PAGE_PROTECTION_FLAGS, VirtualProtect,
};

pub fn hook_addr(addr: *const (), target: *const ()) -> windows::core::Result<*const ()> {
    const ASSEMBLE_SIZE: usize = 4;
    const FLAG: PAGE_PROTECTION_FLAGS = PAGE_EXECUTE_WRITECOPY;
    let mut old_flag = Default::default();
    unsafe { VirtualProtect(addr as _, ASSEMBLE_SIZE, FLAG, &mut old_flag) }?;

    let old = unsafe { assemble_addr(addr, target) };
    let mut unuse = Default::default();
    unsafe { VirtualProtect(addr as _, ASSEMBLE_SIZE, old_flag, &mut unuse) }?;

    Ok(old as _)
}

#[allow(dead_code)]
pub fn hook_near_target(addr: *const (), target: *const ()) -> windows::core::Result<*const ()> {
    let mut old_flag = Default::default();
    unsafe { VirtualProtect(addr as _, 5, PAGE_EXECUTE_WRITECOPY, &mut old_flag) }?;

    let old = unsafe { assemble_near_target(addr as usize, target as usize) };
    let mut unuse = Default::default();
    unsafe { VirtualProtect(addr as _, 5, old_flag, &mut unuse) }?;

    Ok(old as _)
}

unsafe fn assemble_addr(addr: *const (), target: *const ()) -> usize {
    let addr = addr as *mut usize;
    let old_value = unsafe { read_unaligned(addr) };
    unsafe { write_unaligned(addr, target as usize) };
    old_value
}

unsafe fn assemble_near_target(addr: usize, target: usize) -> usize {
    debug_assert_eq!(unsafe { *(addr as *const u8) }, 0xe8);

    let jump_base_addr = addr.wrapping_add(5) as i64;
    let p_jump_target = addr.wrapping_add(1) as *mut i32;
    let old_value = unsafe { read_unaligned(p_jump_target) };

    unsafe { write_unaligned(p_jump_target, (target as i64 - jump_base_addr) as i32) };
    (jump_base_addr + old_value as i64) as usize
}
