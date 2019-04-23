// デカルト座標
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
}

// `Dimension` を `CartesianCoord` に実装する
trait Dimension {
    const DIMENSION: u32;
}

fn main() {
    // 実装された型から定数を取り出す
    #[allow(unused_variables)]
    let dim = CartesianCoord::DIMENSION;

    #[allow(dead_code)]
    const DIM: u32 = CartesianCoord::DIMENSION;
}

// 09
impl Dimension for CartesianCoord {
    const DIMENSION: u32 = 2;
}
