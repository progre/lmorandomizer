use std::ptr::{NonNull, read_unaligned, write_unaligned};

use windows::Win32::System::Memory::{
    PAGE_EXECUTE_WRITECOPY, PAGE_PROTECTION_FLAGS, VirtualProtect,
};

pub fn hook_addr(addr: NonNull<()>, target: *const ()) -> windows::core::Result<*const ()> {
    const ASSEMBLE_SIZE: usize = 4;
    const FLAG: PAGE_PROTECTION_FLAGS = PAGE_EXECUTE_WRITECOPY;
    let mut old_flag = Default::default();
    unsafe { VirtualProtect(addr.as_ptr() as _, ASSEMBLE_SIZE, FLAG, &mut old_flag) }?;

    let old = unsafe { assemble_addr(addr, target) };
    let mut unuse = Default::default();
    unsafe { VirtualProtect(addr.as_ptr() as _, ASSEMBLE_SIZE, old_flag, &mut unuse) }?;

    Ok(old as _)
}

pub fn hook_near_target(
    addr: NonNull<()>,
    target: *const (),
) -> windows::core::Result<NonNull<()>> {
    let mut old_flag = Default::default();
    unsafe { VirtualProtect(addr.as_ptr() as _, 5, PAGE_EXECUTE_WRITECOPY, &mut old_flag) }?;

    let old = unsafe { assemble_near_target(addr, target as usize) };
    let mut unuse = Default::default();
    unsafe { VirtualProtect(addr.as_ptr() as _, 5, old_flag, &mut unuse) }?;

    Ok(NonNull::new(old as _).unwrap())
}

unsafe fn assemble_addr(addr: NonNull<()>, target: *const ()) -> usize {
    let addr = addr.as_ptr() as *mut usize;
    let old_value = unsafe { read_unaligned(addr) };
    unsafe { write_unaligned(addr, target as usize) };
    old_value
}

unsafe fn assemble_near_target(addr: NonNull<()>, target: usize) -> usize {
    debug_assert_eq!(unsafe { *(addr.as_ptr() as *const u8) }, 0xe8);

    let jump_base_addr = unsafe { addr.byte_add(5) }.as_ptr() as i64;
    let p_jump_target = unsafe { addr.byte_add(1) }.as_ptr() as *mut i32;
    let old_value = unsafe { read_unaligned(p_jump_target) };

    unsafe { write_unaligned(p_jump_target, (target as i64 - jump_base_addr) as i32) };
    (jump_base_addr + old_value as i64) as usize
}
