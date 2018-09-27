import assert from 'assert';

const KEY = 0b01100001;

const CODE_MAP: ReadonlyArray<Readonly<{
  code: number;
  char: string;
}>> = (
  // tslint:disable-next-line:prefer-template
  '０１２３４５６７８９\nＢＣＤＥＦ'
  + 'ＳｄＯ新⑩倍母天道書者闇死地古文'
  + ` !"#$%&'()*+,-./`
  + '0123456789:;<=>?'
  + '@ABCDEFGHIJKLMNO'
  + 'PQRSTUVWXYZ[\\]^_'
  + '`abcdefghijklmno'
  + 'pqrstuvwxyz{|}~代'
  + '形勇気年杯体をぁぃぅぇぉゃゅょっ'
  + '真あいうえおかきくけこさしすせそ'
  + '実｡｢｣､･ｦｧｨｩｪｫｬｭｮｯ'
  + 'ｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿ'
  + 'ﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏ'
  + 'ﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝﾞﾟ'
  + 'たちつてとなにぬねのはひふへほま'
  + 'みむめもやゆよらりるれろわん我▼'
)
  .split('')
  .map((x, i) => ({
    code: i,
    char: x,
  }));

const CHAR_TO_CODE: { [char: string]: number }
  // tslint:disable-next-line:prefer-object-spread
  = CODE_MAP.reduce((p, c) => ({ ...p, [c.char]: c.code }), {});
const CODE_TO_CHAR: { [code: number]: string }
  // tslint:disable-next-line:prefer-object-spread
  = CODE_MAP.reduce((p, c) => Object.assign(p, { [c.code]: c.char }), {});

export function decode(bin: ArrayBuffer) {
  let str = '';
  for (const item of new Uint8Array(bin)) {
    str += CODE_TO_CHAR[item ^ KEY];
  }
  return str;
}

export function encode(txt: string, buf: Uint8Array) {
  for (let i = 0; i < txt.length; i += 1) {
    buf[i] = CHAR_TO_CODE[txt.charAt(i)] ^ KEY;
  }
}

export function textToShopData(text: string) {
  return Uint8Array.from(
    text
      .split('')
      .map(x => CHAR_TO_CODE[x]),
  );
}

export function shopItemDataToText(shopItemData: Uint8Array) {
  assert.equal(shopItemData.length, 7 * 3);
  return [...shopItemData].map(x => CODE_TO_CHAR[x]).join('');
}
