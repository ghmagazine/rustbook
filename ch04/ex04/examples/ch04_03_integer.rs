#[allow(unused_variables)]
fn main() {
    let n1 = 10_000;     // i32型（整数リテラルのデフォルトの型）
    let n2 = 0u8;        // u8型（サフィックスで型を指定）
    let n3 = -100_isize; // isize型（同上）

    // 型推論が働く例
    let n4 = 10;         // n4はisize型になる。なぜなら、
    let n5 = n3 + n4;    // ここでisize型のn3に加算しているから

    let h1 = 0xff;                  // i32型、16進数
    let o1 = 0o744;                 // i32型、8進数
    let b1 = 0b1010_0110_1110_1001; // i32型、2進数

    let n6 = b'A';  // ASCII文字'A'の文字コード65u8を得る
    assert_eq!(n6, 65u8);
}
