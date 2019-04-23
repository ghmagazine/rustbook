fn main() {
    let x = 1;
    // 参照からconstなポインタが作れる
    let xptr: *const i32 = &x;
    // 逆の変換はできない
    // let xref: &i32 = xptr;
    // ポインタヘの操作は基本的にアンセーフ
    unsafe {
        // ポインタの参照外しはアンセーフ
        let x = *xptr;
    }

    let mut y = 2;
    // ミュータブルな参照からミュータブルな参照が作れる
    let yptr: *mut i32 = &mut y;
    unsafe {
        // 書き込みももちろんアンセーフ
        *yptr = 3;
    }

    let z = Box::new(4);
    // `Box`を参照してポインタも作れる
    let zptr: *const i32 = &*z;

    let s: &[u8] = b"abc";
    // スライス(文字列)からポインタが作れる
    let sptr: *const u8 = s.as_ptr();
    unsafe {
        // ポインタからスライス(文字列)も作れるが、こちらはアンセーフ。
        let s = std::slice::from_raw_parts(sptr, s.len());
    }
}
