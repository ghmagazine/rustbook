fn main() {
    let i1 = 42; // i32型
    #[allow(unused_variables)]
    let f1 = i1 as f64 / 2.5;  // i32型からf64型へキャスト

    let c1 = 'a';
    assert_eq!(97, c1 as u32); // char型からu32型へキャスト

    let i2 = 300;       // i32型
    let u1 = i2 as u8;  // u8型へキャスト

    // 300はu8型の最大値を超えているので桁あふれして44になる（300を256で割った余りは44）
    assert_eq!(44, u1);
}
