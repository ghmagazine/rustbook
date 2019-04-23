#[derive(Debug)]
enum Either<A, B> {
    A(A),
    B(B),
}

use std::fmt;

impl<A, B> fmt::Display for Either<A, B>
where
    A: fmt::Display,
    B: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Either::A(a) => a.fmt(f),
            Either::B(b) => b.fmt(f),
        }
    }
}

fn main() {
    // `Vec<Either<bool, i32>>` として宣言しておく
    let mut v: Vec<Either<bool, i32>> = vec![];
    // Eitherの値を入れる
    v.push(Either::A(true));
    v.push(Either::B(1i32));

    // すると `{}` で表示できる
    for e in v {
        println!("{}", e);
    }
}
