use libc::{suseconds_t, time_t};
use std::mem;
use std::os::raw::c_int;
use std::ptr;

// #[repr(C)]をつけることでCと相互運用できる型になる。
// メモリ上の表現がC互換になるというだけで、それ以外は普通のRustの構造体として扱える。
// struct timeval {
//     time_t      tv_sec;     /* seconds */
//     suseconds_t tv_usec;    /* microseconds */
// };
#[repr(C)]
#[derive(Debug)]
struct Timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

// struct timezone {
//     int tz_minuteswest;     /* minutes west of Greenwich */
//     int tz_dsttime;         /* type of DST correction */
// };
#[repr(C)]
#[derive(Debug)]
struct Timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

extern "C" {
    // 上記で定義した型をFFIの型に使える。
    // int gettimeofday(struct timeval *tv, struct timezone *tz);
    fn gettimeofday(tv: *mut Timeval, tz: *mut Timezone) -> c_int;
}

fn main() {
    unsafe {
        // Cによって初期化するメモリは`mem::uninitialized`で確保できる。
        // もちろん、Rustの構造体の初期化構文も使える。
        let mut tv: Timeval = mem::uninitialized();
        // あるいはNULLを渡したい場合は`ptr::null_mut`も使える。
        let tz: *mut Timezone = ptr::null_mut();
        let ret = gettimeofday(&mut tv as *mut _, tz);
        if ret == -1 {
            println!("failure");
            return;
        }
        println!("{:?}", tv);
    }
}

