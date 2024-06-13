use rand::Rng;

fn next(rng: &mut impl Rng) -> f64 {
    rng.next_u64() as f64 / u64::MAX as f64
}

pub fn select_random(biases: &[usize], rng: &mut impl Rng) -> usize {
    let mut r = next(rng) * biases.iter().sum::<usize>() as f64;
    for (i, &bias) in biases.iter().enumerate() {
        if r < bias as f64 {
            return i;
        }
        r -= bias as f64;
    }
    unreachable!()
}

pub fn shuffle_simply<T>(list: &mut [T], rng: &mut impl Rng) {
    for i in (0..list.len()).rev() {
        let rand = (next(rng) * (i + 1) as f64) as usize;
        list.swap(i, rand);
    }
}
