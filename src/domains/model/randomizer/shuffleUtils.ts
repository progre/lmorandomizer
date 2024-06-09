import { prng } from 'seedrandom';

export function selectRandom(biases: number[], rng: prng) {
  let r = rng() * biases.reduce((p, c) => p + c, 0);
  for (const [i, bias] of biases.entries()) {
    if (r < bias) {
      return i;
    }
    r -= bias;
  }
  console.error(biases);
  throw new Error();
}

export function shuffleSimply<T>(list: T[], rng: prng) {
  for (let i = list.length - 1; i >= 0; i -= 1) {
    const rand = rng() * (i + 1) | 0;
    [list[i], list[rand]] = [list[rand], list[i]];
  }
}
