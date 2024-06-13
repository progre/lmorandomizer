function next(rng: number[]) {
  return rng.shift() ?? (() => { throw new Error("empty"); })();
}

export function selectRandom(biases: number[], rng: number[]) {
  let r = next(rng) * biases.reduce((p, c) => p + c, 0);
  for (const [i, bias] of biases.entries()) {
    if (r < bias) {
      return i;
    }
    r -= bias;
  }
  console.error(biases);
  throw new Error();
}

export function shuffleSimply<T>(list: T[], rng: number[]) {
  for (let i = list.length - 1; i >= 0; i -= 1) {
    const rand = next(rng) * (i + 1) | 0;
    [list[i], list[rand]] = [list[rand], list[i]];
  }
}
