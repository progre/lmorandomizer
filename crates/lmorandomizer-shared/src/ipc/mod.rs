mod listener;
mod stream;

pub use listener::IpcListener;
pub use stream::IpcStream;

/// Rust の `&str` を Win32 が要求する UTF-16 + null 終端列に変換
fn to_wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
