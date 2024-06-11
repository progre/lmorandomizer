use std::collections::HashMap;

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

pub fn decode(from: &[u8]) -> String {
    let code_to_char = create_code_to_char();
    debug_assert_ne!(from[0], 0);
    let to: String = (0..from.len())
        .map(|i| {
            let query = from[i] ^ KEY;
            code_to_char[&query]
        })
        .collect();
    debug_assert_eq!(to.chars().next(), to.chars().nth(16));
    debug_assert_ne!(to.chars().next(), to.chars().nth(1));
    to
}

pub fn encode(from: &str) -> Vec<u8> {
    let char_to_code = create_char_to_code();
    from.chars()
        .map(|c| char_to_code[&c])
        .map(|ascii| ascii ^ KEY)
        .collect()
}

pub fn text_to_shop_data(text: &str) -> Vec<u8> {
    let char_to_code = create_char_to_code();
    text.chars().map(|c| char_to_code[&c]).collect()
}

pub fn shop_item_data_to_text(shop_item_data: &[u8]) -> String {
    debug_assert_eq!(shop_item_data.len(), 7 * 3);
    shop_item_data
        .iter()
        .map(|&x| CODE_MAP.chars().nth(x as usize).unwrap())
        .collect()
}
