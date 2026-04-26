mod equipment;
mod flags;
mod others;
mod rom;

pub use equipment::Equipment;
pub use flags::Flags;
pub use others::{FieldNumber, MainWeapon, Seal, SubWeapon};
pub use rom::Rom;

pub const MEMO_FLAG_BASE_NO: u16 = 7500;

#[repr(C)]
pub struct Object {
    _padding00: [u32; 4],
    _padding10: [u32; 1],
    pub x: u32,
    pub y: u32,
    _padding1a: [u32; 1],
    _padding20: [u32; 4],
    _padding30: [u32; 3],
    pub op1: i32,
    pub op2: i32,
    pub op3: i32,
    pub op4: i32,
    _padding4a: [u32; 1],
}
