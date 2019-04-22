#[allow(dead_code)]
// #[derive(Default)]
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            stroke_width: 1,               // デフォルト値を1にする
            vertexes: Default::default(),  // Vec<(i32, i32)>のDefault実装を使う
            fill: Default::default(),      // (u8, u8, u8)のDefault実装を使う
        }
    }
}

fn main() {
    // すべてのフィールドがデフォルト値を持つPolygonを作成する
    #[allow(unused_variables)]
    let polygon1: Polygon = Default::default();

    // vertexesフィールドだけ別の値に設定し、他はデフォルト値にする
    #[allow(unused_variables)]
    let polygon2 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        .. Default::default()
    };
}
