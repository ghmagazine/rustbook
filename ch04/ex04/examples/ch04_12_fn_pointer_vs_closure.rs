fn main() {
    let x = 4; // 変数xを4に束縛する

    // クロージャを定義する。するとxがクロージャの環境に捕捉される（キャプチャされる）
    let adder = |n| n + x;
    assert_eq!(adder(2), 4 + 2);

    let mut state = false;
    // 別のクロージャを定義する。このクロージャは引数を取らない
    let mut flipflop = || {
        // stateが補足される
        state = !state; // 状態を反転する
        state
    };

    // クロージャを呼ぶたびに返る値が反転する
    assert!(flipflop());  // true
    assert!(!flipflop()); // false
    assert!(flipflop());  // true

    // クロージャが返す値だけでなく、stateの値も変化している
    assert!(state); // true

    let b = 5;

    // クロージャは1つ1つが独自の匿名の型を持つため、変数fの型はこのクロージャの匿名型になる
    #[allow(unused_mut)]
    let mut f = |a| a * 3 + b;
    // 別のクロージャでは変数fと型が合わず、コンパイルエラーになる
    // f = |a| a * 4 + b;
    // → error[E0308]: mismatched types（型の不一致）
    //   = note: expected type `[closure@src/main.rs:5:17: 5:30 b:_]`
    //              found type `[closure@src/main.rs:6:9: 6:22 b:_]`
    //   = note: no two closures, even if identical, have the same type
    //     （2つのクロージャは、たとえ見た目が同じでも、同じ型を持つことはない）
    f(6);

    // 環境になにも捕捉しないクロージャは関数ポインタ型になれる
    #[allow(unused_mut)]
    let mut f: fn(i32) -> i32 = |n| n * 3;
    assert_eq!(f(-42), -126);

    // 環境になにかを捕捉するクロージャは関数ポインタ型になれない
    #[allow(unused_variables)]
    let x = 4;
    // f = |n| n * x;  // xを捕捉している
    // → error[E0308]: mismatched types
    //   expected fn pointer, found closure
    //  （関数ポインタを期待しているのに、クロージャが見つかった）

    let v = vec!["I", "love", "Rust!"]
        .into_iter()
        // .map(|s| s.len())  // &str型の引数sを取るクロージャ

        // len()メソッドは&str型の引数を1つだけ取るので、len()メソッドへの
        // 関数ポインタでも型が一致する
        .map(str::len)
        .collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 5]);
}
