use std::collections::HashMap;

const KEY: u8 = 0b01100001;
const CODE_MAP: &str = concat!(
    "␀␁␂␃␄␅␆␇␈␉\n␋␌␍␎␏",
    "ＳｄＯ新⑩倍母天道書者闇死地古文",
    " !名魔生命空'目星月夜,-./",
    "0123456789:印<=>?",
    "人ABCDEFGHIJKLMNO",
    "PQRSTUVWXYZ[剣]女男",
    "巨abcdefghijklmno",
    "pqrstuvwxyz汝聖世時代",
    "形勇気年杯体をぁぃぅぇぉゃゅょっ",
    "真あいうえおかきくけこさしすせそ",
    "実｡｢｣､･ｦｧｨｩｪｫｬｭｮｯ",
    "ｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿ",
    "ﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏ",
    "ﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝﾞﾟ",
    "たちつてとなにぬねのはひふへほま",
    "みむめもやゆよらりるれろわん我▼"
);

pub fn code_map() -> Vec<char> {
    CODE_MAP.chars().collect()
}

pub fn reverse_code_map() -> HashMap<char, u8> {
    code_map()
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i as u8))
        .collect()
}

pub fn cipher_to_text(from: &[u8]) -> String {
    debug_assert_ne!(from[0], 0);
    let code_map = code_map();
    let decrypted: String = from.iter().map(|x| code_map[(x ^ KEY) as usize]).collect();
    debug_assert_eq!(decrypted.chars().next(), decrypted.chars().nth(16));
    debug_assert_ne!(decrypted.chars().next(), decrypted.chars().nth(1));
    decrypted
}

pub fn text_to_cipher(from: &str) -> Vec<u8> {
    let char_to_code = reverse_code_map();
    from.chars().map(|c| char_to_code[&c] ^ KEY).collect()
}
