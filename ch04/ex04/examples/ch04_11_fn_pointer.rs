// この関数は引数を2倍した値を返す
fn double(n: i32) -> i32 {
    n + n
}

// この関数は引数の絶対値を返す
fn abs(n: i32) -> i32 {
    if n >= 0 { n } else { -n }
}

fn main() {
    // 変数に型注釈として関数ポインタ型を指定することで、関数名から関数ポインタを得られる。
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84); // double関数で2倍された

    f = abs;
    assert_eq!(f(-42), 42); // abs関数で絶対値を得た

    // 関数ポインタのサイズはusizeと同じ（x86_64アーキテクチャなら8バイト）
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

    // 変数に型注釈を付けないと関数ポインタ型（fn pointer）ではなく
    // 関数定義型（fn item）だと推論される
    #[allow(unused_mut)]
    let mut f_bad = double;

    // 関数定義型は関数ごとに異なる型になるので、変数f_badに別の関数定義型を
    // 束縛できない
    // f_bad = abs;  // 型が合わず、コンパイルエラーになる
    // error[E0308]: mismatched types（型の不一致）
    //    = note: expected type `fn(i32) -> i32 {double}`
    //               found type `fn(i32) -> i32 {abs}`

    // 関数定義型の値のサイズは0バイト
    assert_eq!(std::mem::size_of_val(&f_bad), 0);
}
