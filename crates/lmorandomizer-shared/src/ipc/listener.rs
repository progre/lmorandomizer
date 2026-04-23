use std::io::{self};

use smol::{stream::StreamExt, unblock};
use tracing::debug;
use windows::{
    Win32::{
        Foundation::{ERROR_NO_DATA, ERROR_PIPE_CONNECTED, HANDLE},
        Storage::FileSystem::PIPE_ACCESS_DUPLEX,
        System::Pipes::{
            ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_TYPE_BYTE, PIPE_WAIT,
        },
    },
    core::{Owned, PCWSTR},
};

use super::{IpcStream, to_wide_null};

const PIPE_BUFFER_SIZE: u32 = 1024;

pub struct IpcListener {
    pipe_name: Vec<u16>,
}

impl IpcListener {
    pub fn new(name: &str) -> Self {
        let pipe_name = to_wide_null(&format!(r"\\.\pipe\{name}"));
        Self { pipe_name }
    }

    pub fn incoming(&self) -> impl smol::stream::Stream<Item = io::Result<IpcStream>> {
        smol::stream::repeat_with(|| self.accept()).then(|fut| fut)
    }

    pub async fn accept(&self) -> io::Result<IpcStream> {
        let handle = unsafe {
            Owned::new(CreateNamedPipeW(
                PCWSTR(self.pipe_name.as_ptr()),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                1,
                PIPE_BUFFER_SIZE,
                PIPE_BUFFER_SIZE,
                0,    // デフォルトタイムアウト
                None, // セキュリティ属性なし
            ))
        };
        if handle.is_invalid() {
            return Err(io::Error::last_os_error());
        }

        debug!("Pipe created, waiting for client to connect...");

        // クライアントの接続を同期的に待機
        let raw_handle = handle.0 as usize;
        match unblock(move || unsafe { ConnectNamedPipe(HANDLE(raw_handle as _), None) }).await {
            Ok(()) => {}
            Err(e) => {
                if e.code() != ERROR_PIPE_CONNECTED.to_hresult()
                    && e.code() != ERROR_NO_DATA.to_hresult()
                {
                    return Err(io::Error::from_raw_os_error(e.code().0));
                }
            }
        }

        Ok(IpcStream::new(handle))
    }
}
