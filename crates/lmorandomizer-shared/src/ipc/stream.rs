use std::io::{self, Read, Write};

use tracing::{error, trace};
use windows::{
    Win32::{
        Foundation::{GENERIC_READ, GENERIC_WRITE, HANDLE},
        Storage::FileSystem::{
            CreateFileW, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE, FlushFileBuffers,
            OPEN_EXISTING, ReadFile, WriteFile,
        },
        System::Pipes::{DisconnectNamedPipe, WaitNamedPipeW},
    },
    core::{Owned, PCWSTR},
};

use super::to_wide_null;

pub struct IpcStream {
    handle: Owned<HANDLE>,
}

impl IpcStream {
    pub fn new(handle: Owned<HANDLE>) -> Self {
        Self { handle }
    }

    pub fn connect(name: &str) -> io::Result<Self> {
        let pipe_name = to_wide_null(&format!(r"\\.\pipe\{name}"));

        loop {
            let handle = unsafe {
                CreateFileW(
                    PCWSTR(pipe_name.as_ptr()),
                    (GENERIC_READ | GENERIC_WRITE).0,
                    FILE_SHARE_MODE::default(),
                    None,
                    OPEN_EXISTING,
                    FILE_FLAGS_AND_ATTRIBUTES::default(),
                    None,
                )
            };
            match handle {
                Ok(h) => return Ok(Self::new(unsafe { Owned::new(h) })),
                Err(err) => {
                    eprintln!("Failed to connect to pipe: {}. Retrying...", err);
                    // パイプが使用中なら待機して再試行
                    let wait_result = unsafe { WaitNamedPipeW(PCWSTR(pipe_name.as_ptr()), 5000) };
                    if !wait_result.as_bool() {
                        return Err(io::Error::last_os_error());
                    }
                }
            }
        }
    }
}

unsafe impl Send for IpcStream {}
unsafe impl Sync for IpcStream {}

impl Drop for IpcStream {
    fn drop(&mut self) {
        if let Err(err) = unsafe { DisconnectNamedPipe(*self.handle) } {
            error!("{err}");
        }
    }
}

impl Read for IpcStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        trace!("read: handle={:?}, buf_len={}", self.handle, buf.len());
        let mut read = 0;
        unsafe { ReadFile(*self.handle, Some(buf), Some(&mut read), None) }
            .map_err(|e| io::Error::from_raw_os_error(e.code().0))?;
        Ok(read as usize)
    }
}

impl Write for IpcStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        unsafe { WriteFile(*self.handle, Some(buf), Some(&mut written), None) }
            .map_err(|e| io::Error::from_raw_os_error(e.code().0))?;
        Ok(written as usize)
    }

    fn flush(&mut self) -> io::Result<()> {
        unsafe {
            FlushFileBuffers(*self.handle).map_err(|e| io::Error::from_raw_os_error(e.code().0))
        }
    }
}
