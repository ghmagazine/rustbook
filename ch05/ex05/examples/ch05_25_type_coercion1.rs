fn main() {
    intro();
    coercion_sites();
    transitivity();
}

fn intro() {
    // 整数リテラル3、4、5は通常はi32型と解釈されるが、
    // 型アノテーション（型注釈）によってu8型へと型強制されている
    let v1: Vec<u8> = vec![3, 4, 5];

    // もし型強制がなかったらこう書かなければならない
    // let v1 = vec![3u8, 4u8, 5u8];

    // Vec<u8>型からスライス&[u8]型へ型強制されることによって
    // スライスに備わったfirst(&self)メソッドが使用できる
    assert_eq!(Some(&3u8), v1.first());

    // もし型強制がなかったらこう書かなければならない
    // assert_eq!(Some(&3u8), (&v1[..]).first());

    let mut s1 = String::from("Type coercion ");
    let s2 = String::from("is actually easy.");

    // push_str()のシグネチャはpush_str(self: &mut String, s: &str)
    // 型強制によってs1がString型から&mut String型へ変換され、
    // &s2は&String型から&str型へ変換される
    s1.push_str(&s2);

    // もし型強制がなかったらこう書かなければならない
    // (&mut s1).push_str(s2.as_str());
}

fn coercion_sites() {
    let mut _i1 = 0u8;  // i1はu8型だと型推論される
    _i1 = 255;          // よって255はu8型へ型強制される
}

fn transitivity() {
    let p1 = &&&&[1, 2, 3, 4];  // &&&&[i32; 4]型

    // 型強制が&&&&a1 → &&&a1 → &&a1 → &a1の順に推移的に作用する
    #[allow(unused_variables)]
    let p2: &[i32; 4] = p1;

    let p3 = &&[1, 2, 3, 4];  // &&[i32; 4]型

    // 配列への参照&&[i32; 4]型からスライス&[i32]型まで段階を踏むと型強制できる
    let p4: &[i32; 4] = p3;
    #[allow(unused_variables)]
    let p5: &[i32] = p4;

    // しかし一度にはできない（rustc 1.31.0）
    // let p6: &[i32] = p3;
    // → error[E0308]: mismatched types
    //   expected type `&[i32]`, found type `&&[i32; 4]`
}
