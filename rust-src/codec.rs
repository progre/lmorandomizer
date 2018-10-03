use std::collections::HashMap;
use std::slice::{from_raw_parts, from_raw_parts_mut};

use helper;

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

#[no_mangle]
pub fn decode(len: usize, from_ascii_ptr: *const u8, to_utf16_ptr: *mut u16) {
    let code_to_char = create_code_to_char();
    let from = unsafe { from_raw_parts(from_ascii_ptr, len) };
    assert_eq!(from.len(), len);
    let to = unsafe { from_raw_parts_mut(to_utf16_ptr, len) };
    assert_eq!(to.len(), len);
    assert_ne!(from[0], 0);
    for i in 0..len {
        let mut b = [0; 1];
        let query = from[i as usize] ^ KEY;
        let utf16 = code_to_char.get(&query).unwrap().encode_utf16(&mut b);
        assert_eq!(utf16.len(), 1);
        to[i as usize] = utf16[0];
    }
    assert_eq!(to[0], to[16]);
    assert_ne!(to[0], to[1]);
}
