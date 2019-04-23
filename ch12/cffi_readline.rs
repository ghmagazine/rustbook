// `String`のCのNULL終端文字列に対応する型。
use std::ffi::CString;
// `str`のCのnull終端文字列に対応する型。
use std::ffi::CStr;
use std::os::raw::c_schar;

// `readline`ライブラリとリンクする。
#[link(name = "readline")]
extern "C" {
    // Cの符号付き `char` 型を`c_schar`で表現している。
    fn readline(prompt: *const c_schar) -> *mut c_schar;
}

fn main() {
    unsafe {
        // Rustの文字列をCの文字列に変換する。
        // NULL終端するために`CString`を使う。
        let prompt = CString::new("> ").unwrap();
        loop {
            // `readline`を呼び、結果を`CStr`でラップする。
            let input = CStr::from_ptr(readline(prompt.as_ptr()));
            // `&CStr`を`&str`に変換する。
            let input = input.to_str().expect("input contains invalid unicode");
            // 以後はRustの文字列なので自由に操作できる。
            if input == "exit" {
                break;
            }

            println!("your input is {}", input);
        }
    }
}
