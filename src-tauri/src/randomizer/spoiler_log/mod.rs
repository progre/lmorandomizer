mod build;
mod display;

/// spoiler log の 1 行。shop は枠ごとに持ち、出力時に同じ shop の枠を 1 行にまとめる
#[derive(Debug)]
enum Line {
    Spot {
        spot: String,
        item: String,
    },
    Shop {
        shop: String,
        idx: usize,
        item: String,
    },
}

#[derive(Debug)]
struct Sphere {
    lines: Vec<Line>,
    events: Vec<String>,
}

#[derive(Debug)]
pub struct SpoilerLog {
    progression: Vec<Sphere>,
    maps: Vec<Line>,
}
