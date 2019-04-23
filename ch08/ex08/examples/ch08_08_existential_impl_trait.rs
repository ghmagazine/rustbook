#[allow(dead_code)]
fn to_n(n: i32) -> impl Iterator {
    0..n
}

// use std::ops::Range;
// #[allow(dead_code)]
// fn to_n(n: i32) -> Range<i32> {
//     0..n
// }

#[allow(dead_code)]
fn to_n_even(n: i32) -> impl Iterator {
    (0..n).filter(|i| i % 2 == 0)
}

use std::fmt;

fn one() -> impl fmt::Display {
    1i32
}

// fn one(is_float: bool) -> impl fmt::Display {
//     // error[E0308]: if and else have incompatible types
//     if is_float {
//         1.0f32
//     } else {
//         1i32
//     }
// }

fn main() {
    let mut _n = one();
    _n = one();
}

#[allow(dead_code)]
fn gen_counter(init: i32) -> impl FnMut() -> i32 {
    let mut n = init;
    move || {
        let ret = n;
        n += 1;
        ret
    }
}
