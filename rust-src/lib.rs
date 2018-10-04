use std::mem;
use std::os::raw::c_void;

pub mod codec;
mod helper;

#[no_mangle]
pub fn init() {
    helper::init();
}

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub fn free(ptr: *mut c_void, size: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, size);
    }
}

pub struct StringBuilder {
    str: String,
}

#[no_mangle]
pub fn create_string_builder_with_capacity(size: i32) -> *mut StringBuilder {
    Box::into_raw(Box::new(StringBuilder {
        str: String::with_capacity(size as usize),
    }))
}

#[no_mangle]
pub fn destroy_string_builder(b: *mut StringBuilder) {
    unsafe {
        Box::from_raw(b);
    }
}

#[no_mangle]
pub fn string_builder_append_unchecked(sb: *mut StringBuilder, code_point: i32) {
    unsafe {
        (*sb)
            .str
            .push(::std::char::from_u32_unchecked(code_point as u32));
    }
}
