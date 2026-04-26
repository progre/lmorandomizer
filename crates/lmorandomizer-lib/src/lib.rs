mod app;
mod hook;
mod hook_utils;
mod tracing_helper;

use std::{panic, process, sync::Arc, thread};

use lmorandomizer_shared::ipc::IpcListener;
use smol::block_on;
use tracing::{debug, error};
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{LibraryLoader::GetModuleHandleW, SystemServices::DLL_PROCESS_ATTACH},
};

use crate::{app::LmoRandomizerHelper, tracing_helper::init_tracing};
use hook::{LmoHandle, install_hook};

#[unsafe(no_mangle)]
pub extern "stdcall" fn DllMain(_inst_dll: HINSTANCE, reason: u32, _reserved: u32) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        thread::spawn(|| block_on(sub_main()));
    }

    true
}

async fn sub_main() {
    init_tracing();
    panic::set_hook(Box::new(|info| {
        error!("panic occurred: {}", info);
    }));
    debug!("sub_main start");

    let base_addr = unsafe { GetModuleHandleW(None) }.unwrap().0 as usize;
    let handle = LmoHandle::new(base_addr);
    let handle_ptr = &handle as *const LmoHandle;
    let helper = Arc::new(LmoRandomizerHelper::new(handle));
    let delegate = helper.clone();
    install_hook(unsafe { handle_ptr.as_ref() }.unwrap(), delegate).unwrap();

    let pipe_name = format!("lmorandomizer_for_{}", process::id());
    let listener = IpcListener::new(&pipe_name);
    let stream = listener.accept().await.unwrap();
    helper.ipc_main(stream).await.unwrap();

    debug!("sub_main end");
}
