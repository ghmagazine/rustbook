#[allow(dead_code)]
#[derive(Default)]
pub struct Polygon<T> {
    pub vertexes: Vec<T>,
    pub stroke_width: u8,
    pub fill: (u8, u8, u8),
    internal_id: String,
}

// 座標
trait Coordinates {}

// デカルト座標
#[allow(dead_code)]
#[derive(Default)]
struct CartesianCoord {
    x: f64,
    y: f64,
}
impl Coordinates for CartesianCoord {}

// 極座標
#[allow(dead_code)]
#[derive(Default)]
struct PolarCoord {
    r: f64,
    theta: f64,
}
impl Coordinates for PolarCoord {}

fn main() {
    let vertexes = vec![
        CartesianCoord {x:  0.0, y:  0.0},
        CartesianCoord {x: 50.0, y:  0.0},
        CartesianCoord {x: 30.0, y: 20.0},
    ];

    #[allow(unused_variables)]
    // Polygon<CartesianCoord>型
    let poly = Polygon { vertexes, .. Default::default() };
}
