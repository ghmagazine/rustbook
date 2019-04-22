fn main() {
    let utf16: Vec<u16> = vec![0x61, 0x62, 0x6f22, 0x5b57];

    // Vec<u16>の値をUTF-16と解釈しStringを作成する（UTF-8へ変換される）
    if let Ok(s) = String::from_utf16(&utf16) {
        assert_eq!(s, "ab漢字");
    } else {
        unreachable!();
    }

    // バイト文字列リテラル。ASCII文字以外のバイトは「\x2桁の16進数」で記述する
    let bs1 = b"abc\xe3\x81\x82";  // &[u8; 6]型。UTF-8表現で"abcあ"
    assert_eq!(bs1, &[b'a', b'b', b'c', 0xe3, 0x81, 0x82]);

    // rawバイト文字列リテラル。エスケープ文字（\）を特別扱いしないので、\nは
    // 改行文字ではなく文字どおり\nと解釈される。
    let bs2 = br#"ab\ncd"#;        // &[u8; 6]型
    assert_eq!(bs2, &[b'a', b'b', b'\\', b'n', b'c', b'd']);
}
