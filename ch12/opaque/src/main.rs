use std::os::raw::{c_char, c_int};

// オペーク型を表わす型を導入する。
// バリアントのない列挙型は値が作れないのでユーザが勝手にインスタンスを作ることはできない。
// この列挙型へのポインタでオペーク型へのポインタを表す。
enum File {}

extern "C" {
    // Cの`FILE`型の実体が分からないのでRust側では実体に言及しない型でマッピングする。

    // FILE *fopen(const char *path, const char *mode);
    fn fopen(fname: *const c_char, mode: *const c_char) -> *mut File;

    // int fgetc(FILE *stream);
    fn fgetc(stream: *mut File) -> c_int;

    // int fclose(FILE *stream);
    fn fclose(stream: *mut File) -> c_int;
}

fn main() {
    unsafe {
        // Cの文字列を作る。ここではNULL終端したバイト列を作ってキャストしている。
        let fname: *const c_char = b"Cargo.toml\0".as_ptr() as *const _;
        let mode: *const c_char = b"r\0".as_ptr() as *const _;
        // `FILE`はRustでは本来実体のない型なのでC関数を通してのみ初期化できる。
        let file = fopen(fname, mode);
        if file.is_null() {
            println!("open file failed");
            return;
        }
        loop {
            // Rustにとってはよく分からない値のままCの関数に渡す。
            let c = fgetc(file);
            if c == -1 {
                break;
            } else {
                let c = c as u8 as char;
                print!("{}", c);
            }
        }
        // 同じく実体のよく分からないままCの関数で終了処理をする。
        if fclose(file) == -1 {
            println!("close file failed");
        }
    }
}
