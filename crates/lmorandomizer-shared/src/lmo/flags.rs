use std::ptr::{NonNull, read_volatile};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Flags(#[serde(with = "serde_big_array::BigArray")] [u8; 1000]);

impl Flags {
    pub fn new(value: [u8; 1000]) -> Self {
        Self(value)
    }

    pub fn get(&self, idx: u16) -> bool {
        let byte_index = (idx / 8) as usize;
        let bit_index = (idx % 8) as u8;

        let byte = self.0[byte_index];

        (byte & (1 << bit_index)) != 0
    }

    /// # Safety
    /// `ptr` must be a valid pointer to a `Flags` struct.
    pub unsafe fn get_volatile(ptr: NonNull<Self>, idx: u16) -> bool {
        let byte_index = (idx / 8) as usize;
        let bit_index = (idx % 8) as u8;

        let base = ptr.as_ptr() as *const u8;
        let byte = unsafe { read_volatile(base.add(byte_index)) };

        (byte & (1 << bit_index)) != 0
    }
}
