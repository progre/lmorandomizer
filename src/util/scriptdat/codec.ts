import assert from 'assert';
import iconv from 'iconv-lite';

const KEY = 0b01100001;

const CODE_MAP: ReadonlyArray<Readonly<{
  code: number;
  char: string;
}>> = (() => {
  const src = [
    0x00, '０１２３４５６７８９\nｂｃｄｅｆ',
    0x10, 'ＳｄＯ新⑩倍母天道書者闇死地古文',
    0x7F, '代形勇気年杯体をぁぃぅぇぉゃゅょっ'
    + '真あいうえおかきくけこさしすせそ'
    + '実',
    0xE0, 'たちつてとなにぬねのはひふへほま'
    + 'みむめもやゆよらりるれろわん我▼',
  ];
  const list: Readonly<{
    code: number;
    char: string;
  }>[] = [];
  for (let i = 0; i < src.length; i += 2) {
    (<string>src[i + 1])
      .split('')
      .map((x, j) => ({
        code: (<number>src[i]) + j,
        char: x,
      }))
      .forEach((x) => {
        list.push(x);
      });
  }
  return list;
})();

export async function decode(bin: ArrayBuffer): Promise<string> {
  let str = '';
  for (const [i, item] of new Uint8Array(bin).entries()) {
    str += toChar(item ^ KEY);
    if (i % 1000 === 0) {
      await new Promise((resolve) => {
        setImmediate(resolve);
      });
    }
  }
  return str;
}

export async function encode(txt: string) {
  const array: number[] = [];
  for (const [i, char] of txt.split('').entries()) {
    array.push(toCode(char) ^ KEY);
    if (i % 1000 === 0) {
      await new Promise((resolve) => {
        setImmediate(resolve);
      });
    }
  }
  return Uint8Array.from(array);
}

export function textToShopData(text: string) {
  return Uint8Array.from(
    text
      .split('')
      .map(toCode),
  );
}

function toCode(char: string) {
  const idx = CODE_MAP.findIndex(y => y.char === char);
  if (idx < 0) {
    return iconv.encode(char, 'Shift_JIS')[0];
  }
  return CODE_MAP[idx].code;
}

export function shopItemDataToText(shopItemData: Uint8Array) {
  assert.equal(shopItemData.length, 7 * 3);
  return [...shopItemData].map(toChar).join('');
}

function toChar(code: number) {
  const idx = CODE_MAP.findIndex(y => y.code === code);
  if (idx < 0) {
    return iconv.decode(<any>[code], 'Shift_JIS');
  }
  return CODE_MAP[idx].char;
}
