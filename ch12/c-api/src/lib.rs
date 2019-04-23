use std::os::raw::{c_double, c_int};

// Cのデータの扱いで説明したときと同様 `#[repr(C)]` で互換性を取れる
/// 2次元平面上の点を表す型
#[repr(C)]
pub struct point {
    x: c_int,
    y: c_int,
}

fn pow(x: c_int) -> c_int {
    x * x
}

// `#[no_mangle]`をつけることでCから"dist"という名前で見つかるようになる。
// `extern "C"`をつけることでCから呼べるようになる。
/// 2点間の距離を計算する
#[no_mangle]
pub extern "C" fn dist(p1: &point, p2: &point) -> c_double {
    let d = pow(p1.x - p2.x) + pow(p1.y - p2.y);
    (d as f64).sqrt() as c_double
}

