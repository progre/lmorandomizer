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

export function shuffleSimply<T>(list: ReadonlyArray<T>, rng: prng): ReadonlyArray<T> {
  const array = [...list];
  for (let i = array.length - 1; i >= 0; i -= 1) {
    const rand = Math.floor(rng() * (i + 1));
    [array[i], array[rand]] = [array[rand], array[i]];
  }
  return array;
}
