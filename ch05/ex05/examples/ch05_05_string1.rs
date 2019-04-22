fn main() {
    // strリテラルからStringを作る。どちらの方法でも結果は同じ
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    // Rust 1.19より前のバージョンでは性能上の理由からto_string()よりも
    // to_owned()が推奨されていた。現在のバージョンではそのような配慮は不要
    let s3 = "ストロベリー".to_owned();

    s1.push_str("タルト");    // String型の文字列にに&str型の文字列を追加
    assert_eq!(s1, "ラズベリータルト");

    s2.push('と');           // Stringにcharを追加する

    // push_str()が受け付けるのは&str型のみ。以下はコンパイルエラーになる
    // s2.push_str(s3);         // s3はString型
    // → error[E0308]: mismatched types
    //   expected &str, found struct `std::string::String`

    // &を付けると型強制というしくみによって&Stringから&strへ変換される
    s2.push_str(&s3);
    assert_eq!(s2, "ブラックベリーとストロベリー");
}
