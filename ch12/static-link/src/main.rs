use std::os::raw::{c_int, c_ulonglong};

// kind="static"とすることで静的リンクライブラリをリンクできる
#[link(name = "fib", kind = "static")]
extern "C" {
    fn fib(n: c_int) -> c_ulonglong;
}

fn main() {
    unsafe {
        println!("fib(5) = {}", fib(5));
    }
}

