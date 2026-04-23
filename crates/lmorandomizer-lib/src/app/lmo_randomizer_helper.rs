use std::{
    ffi::{CStr, OsStr, c_char},
    iter,
    os::windows::ffi::OsStrExt,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

use anyhow::{Result, anyhow};
use lmorandomizer_shared::{Command, ipc::IpcStream};
use smol::unblock;
use tracing::trace;
use windows::{
    Win32::{
        Foundation::{HANDLE, INVALID_HANDLE_VALUE},
        Security::SECURITY_ATTRIBUTES,
        Storage::FileSystem::{
            CreateFileW, FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE,
        },
    },
    core::{PCSTR, PCWSTR},
};

use crate::hook::{CreateFileAFn, LmoDelegate};

#[derive(Debug)]
pub struct LmoRandomizerHelper {
    custom_path: OnceLock<PathBuf>,
}

impl LmoRandomizerHelper {
    pub fn new() -> Self {
        Self {
            custom_path: OnceLock::new(),
        }
    }

    pub async fn ipc_main(&self, stream: IpcStream) -> Result<()> {
        let stream = Arc::new(Mutex::new(stream));

        loop {
            let stream = Arc::clone(&stream);
            let Ok(cmd) =
                unblock(move || rmp_serde::from_read::<_, Command>(&mut *stream.lock().unwrap()))
                    .await
            else {
                break;
            };

            trace!("ipc_main while {cmd:?}");
            match cmd {
                Command::Init(path_buf) => {
                    self.custom_path
                        .set(path_buf)
                        .map_err(|_| anyhow!("Failed to set custom path"))?;
                }
            }
        }
        Ok(())
    }
}

impl LmoDelegate for LmoRandomizerHelper {
    fn create_file_a(
        &self,
        lpfilename: PCSTR,
        dwdesiredaccess: u32,
        dwsharemode: FILE_SHARE_MODE,
        lpsecurityattributes: *const SECURITY_ATTRIBUTES,
        dwcreationdisposition: FILE_CREATION_DISPOSITION,
        dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
        htemplatefile: HANDLE,
        original: CreateFileAFn,
    ) -> HANDLE {
        let custom_path = self.custom_path.wait();
        const FILE_MAP: [(&CStr, &str); 6] = [
            (cr"data\script.dat", "script.dat"),
            (cr"lamulana.sa0", "lamulana.sa0"),
            (cr"lamulana.sa1", "lamulana.sa1"),
            (cr"lamulana.sa2", "lamulana.sa2"),
            (cr"lamulana.sa3", "lamulana.sa3"),
            (cr"lamulana.sa4", "lamulana.sa4"),
        ];
        let filename = unsafe { CStr::from_ptr(lpfilename.0.cast::<c_char>()) };
        let Some((_, replacement)) = FILE_MAP.iter().find(|(target, _)| filename == target) else {
            trace!("CreateFileA: {:?} {:?}", custom_path.to_str(), unsafe {
                lpfilename.to_string()
            });
            return original(
                lpfilename,
                dwdesiredaccess,
                dwsharemode,
                lpsecurityattributes,
                dwcreationdisposition,
                dwflagsandattributes,
                htemplatefile,
            );
        };
        let filename = custom_path.join(replacement);
        let filename = to_wide_null(&filename.into_os_string());
        unsafe {
            CreateFileW(
                PCWSTR(filename.as_ptr()),
                dwdesiredaccess,
                dwsharemode,
                Some(lpsecurityattributes),
                dwcreationdisposition,
                dwflagsandattributes,
                Some(htemplatefile),
            )
        }
        .unwrap_or(INVALID_HANDLE_VALUE)
    }
}

fn to_wide_null(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(iter::once(0)).collect()
}
