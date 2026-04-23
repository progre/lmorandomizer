use std::io::{self, Write};

use tracing::Level;
use tracing_subscriber::fmt::MakeWriter;
use windows::{Win32::System::Diagnostics::Debug::OutputDebugStringW, core::HSTRING};

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_writer(MakeDebugViewWriter)
        .without_time()
        .with_ansi(false)
        .with_max_level(if cfg!(debug_assertions) {
            Level::TRACE
        } else {
            Level::ERROR
        })
        .init();
}

struct DebugViewWriter;

impl Write for DebugViewWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = HSTRING::from(String::from_utf8_lossy(buf).as_ref());
        unsafe { OutputDebugStringW(&s) };
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct MakeDebugViewWriter;

impl<'a> MakeWriter<'a> for MakeDebugViewWriter {
    type Writer = DebugViewWriter;

    fn make_writer(&'a self) -> Self::Writer {
        DebugViewWriter
    }
}
