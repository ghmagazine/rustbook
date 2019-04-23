// デカルト座標
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
}

// 極座標
pub struct PolarCoord {
    r: f64,
    theta: f64,
}

// 座標
pub trait Coordinates {
    // 関数の本体は書かない
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

// デカルト座標系はそのまま
impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

// 極座標系は変換が必要
impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

// タプルにもトレイトを実装できる
impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

// fn print_point<P: Coordinates>(point: P) {
//     let p = point.to_cartesian();
//     println!("({}, {})", p.x, p.y)
// }

// fn print_point<P>(point: P)
// where
//     P: Coordinates,
// {
//     let p = point.to_cartesian();
//     println!("({}, {})", p.x, p.y)
// }

fn print_point(point: impl Coordinates) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y)
}

// #[allow(dead_code)]
// fn as_cartesian<P: Coordinates + Clone>(point: &P) -> CartesianCoord {
//     point.clone().to_cartesian()
// }

#[allow(dead_code)]
fn as_cartesian(point: &(impl Coordinates + Clone)) -> CartesianCoord {
    point.clone().to_cartesian()
}

// `P` を2回書くにはジェネリクスが必要
#[allow(dead_code)]
fn double_point<P: Coordinates>(point: P) -> P {
    let mut cart = point.to_cartesian();
    cart.x *= 2.0;
    cart.y *= 2.0;
    P::from_cartesian(cart)
}

// `(T, T)` のように `T` そのものでない型への制約はジェネリクスが必要
#[allow(dead_code)]
fn make_point<T>(x: T, y: T) -> CartesianCoord
where
    (T, T): Coordinates,
{
    (x, y).to_cartesian()
}

// ジェネリックトレイトを用意しておく
// 後に説明するがトレイトもジェネリクスにできる
trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

#[allow(dead_code)]
fn to<T>(i: i32) -> T
where
    // `ConvertTo<T>` と型パラメータがトレイト側にある
    // where 記法だと`i32`など具体的な型に制約がかける
    i32: ConvertTo<T>,
{
    i.convert()
}

fn main() {
    // `Coordinates`を実装している型の値に対して呼べる
    print_point((0.0, 1.0)); // (0, 1)
    print_point(PolarCoord {
        r: 1.0,
        theta: std::f64::consts::PI / 2.0,
    }); // (0.00000000000000006123233995736766, 1)

    // しかし  `Coordinates` を実装していない型の値を引数に渡そうとするとコンパイルエラーになる
    // print_point("string"); // error[E0277]: the trait bound `&str: Coordinates` is not satisfied
}
