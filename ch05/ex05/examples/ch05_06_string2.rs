fn main() {
    let i = 42;           // i32型
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1;    // f64型
    assert_eq!(f.to_string(),       "4.3999999999999995");
    assert_eq!(format!("{:.2}", f), "4.40");  // format!マクロが便利

    let t = (1, "ABC");
    // 2要素のタプル型はDebugトレイトを実装しているのでformat!マクロで変換できる
    assert_eq!(format!("{:?}", t), r#"(1, "ABC")"#);

    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42)); // &str型からi32型へ変換

    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse();   // 変数の型から型推論できるならparseの型パラメータは不要
    assert!(r2.is_err());                  // 数値として解釈できないときはエラーが返る
    println!("{:?}", r2);                  // → Err(ParseFloatError { kind: Invalid })

    let cs = ['t', 'r', 'u', 's', 't'];      // [char; 5]型
    assert_eq!(cs.iter().collect::<String>(),       "trust");
    assert_eq!(&cs[1..].iter().collect::<String>(), "rust" );

    let bad_utf8: [u8; 7] = [
        b'a',              // a
        0xf0, 0x90, 0x80,  // でたらめなバイト列
        0xe3, 0x81, 0x82,  // あ
    ];

    // 不正なバイト列はUnicodeのU+FFFD Replacement Characterに置き換わる
    let s = String::from_utf8_lossy(&bad_utf8);
    assert_eq!(s, "a\u{fffd}あ");
}
