use std::collections::HashMap;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use StringBuilder;

const KEY: u8 = 0b01100001;
const CODE_MAP: &str = concat!(
    "０１２３４５６７８９\nＢＣＤＥＦ",
    "ＳｄＯ新⑩倍母天道書者闇死地古文",
    " !\"#$%&'()*+,-./",
    "0123456789:;<=>?",
    "@ABCDEFGHIJKLMNO",
    "PQRSTUVWXYZ[\\]^_",
    "`abcdefghijklmno",
    "pqrstuvwxyz{|}~代",
    "形勇気年杯体をぁぃぅぇぉゃゅょっ",
    "真あいうえおかきくけこさしすせそ",
    "実｡｢｣､･ｦｧｨｩｪｫｬｭｮｯ",
    "ｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿ",
    "ﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏ",
    "ﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝﾞﾟ",
    "たちつてとなにぬねのはひふへほま",
    "みむめもやゆよらりるれろわん我▼"
);

fn create_code_to_char() -> HashMap<u8, char> {
    return CODE_MAP
        .chars()
        .enumerate()
        .map(|(i, x)| (i as u8, x))
        .collect::<HashMap<_, _>>();
}

fn create_char_to_code() -> HashMap<char, u8> {
    return CODE_MAP
        .chars()
        .enumerate()
        .map(|(i, x)| (x, i as u8))
        .collect::<HashMap<_, _>>();
}

#[no_mangle]
pub fn decode(len: usize, from_ascii_ptr: *const u8, to_utf16_ptr: *mut u16) {
    let code_to_char = create_code_to_char();
    let from = unsafe { from_raw_parts(from_ascii_ptr, len) };
    debug_assert_eq!(from.len(), len);
    let to = unsafe { from_raw_parts_mut(to_utf16_ptr, len) };
    debug_assert_eq!(to.len(), len);
    debug_assert_ne!(from[0], 0);
    for i in 0..len {
        let mut b = [0; 1];
        let query = from[i as usize] ^ KEY;
        let utf16 = code_to_char.get(&query).unwrap().encode_utf16(&mut b);
        debug_assert_eq!(utf16.len(), 1);
        to[i as usize] = utf16[0];
    }
    debug_assert_eq!(to[0], to[16]);
    debug_assert_ne!(to[0], to[1]);
}

#[no_mangle]
pub fn encode(from_sb: *mut StringBuilder, len: usize, to_ascii_ptr: *mut u8) {
    let from: &str = unsafe { &(*from_sb).str };
    let char_to_code = create_char_to_code();
    debug_assert_eq!(from.chars().count(), len);
    let to = unsafe { from_raw_parts_mut(to_ascii_ptr, len) };
    debug_assert_eq!(to.len(), len + 1);
    for (i, ascii) in from
        .chars()
        .map(|c| char_to_code.get(&c).unwrap())
        .enumerate()
    {
        to[i as usize] = ascii ^ KEY;
    }
}
