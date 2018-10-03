use std::panic;

extern "C" {
    fn js_console_log(ptr: *const u16, size: usize);
    fn js_console_error(ptr: *const u16, size: usize);
}

pub fn console_log(message: &str) {
    let str_utf16: Vec<u16> = message.encode_utf16().collect();
    unsafe { js_console_log(str_utf16.as_ptr(), str_utf16.len() * 2) };
}

pub fn console_error(message: &str) {
    let str_utf16: Vec<u16> = message.encode_utf16().collect();
    unsafe { js_console_error(str_utf16.as_ptr(), str_utf16.len()) };
}

pub fn init() {
    panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();

        let payload = if payload.is::<String>() {
            Some(payload.downcast_ref::<String>().unwrap().as_str())
        } else if payload.is::<&str>() {
            Some(*payload.downcast_ref::<&str>().unwrap())
        } else {
            None
        };
        if let (Some(payload), Some(location)) = (payload, panic_info.location()) {
            console_error(
                format!(
                    "panicked at {:?}, {}:{}:{}",
                    payload,
                    location.file(),
                    location.line(),
                    location.column()
                ).as_str(),
            );
        }
    }));
}
