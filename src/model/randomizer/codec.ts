import iconv from 'iconv-lite';

const KEY = 0b01100001;

const CODE_MAP: ReadonlyArray<Readonly<{
  code: number;
  char: string;
}>> = (() => {
  const src = [
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
    const code = item ^ KEY;
    const idx = CODE_MAP.findIndex(y => y.code === code);
    if (idx < 0) {
      str += iconv.decode(<any>[code], 'Shift_JIS');
    } else {
      str += CODE_MAP[idx].char;
    }
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
    const idx = CODE_MAP.findIndex(y => y.char === char);
    let code;
    if (idx < 0) {
      code = iconv.encode(char, 'Shift_JIS')[0];
    } else {
      code = CODE_MAP[idx].code;
    }
    array.push(code ^ KEY);
    if (i % 1000 === 0) {
      await new Promise((resolve) => {
        setImmediate(resolve);
      });
    }
  }
  return Uint8Array.from(array);
}
