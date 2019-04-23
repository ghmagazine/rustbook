use std::os::raw::c_int;

#[link(name = "readline")]
extern "C" {
    // rustの`static`と同じく`static 名前: 型;`で宣言します。
    static rl_readline_version: c_int;
}

fn main() {
    unsafe {
        // readlineのバージョンは16進数なので `:x` で16進表示する。
        println!("using readline version {:x}", rl_readline_version);
    }
}
