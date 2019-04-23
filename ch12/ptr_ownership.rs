fn main() {
    let boxed = Box::new(true);
    // ここでboxedの所有権はムーブしてしまう
    let ptr: *mut bool = Box::into_raw(boxed);
    unsafe {
        // ポイント先のメモリを解放するにはBox::from_rawでBoxに戻してあげる必要がある
        // ここでポインタのデータ型の所有権をboxedが持つことになる
        // 他に参照がないかはユーザが保証する必要がある。
        let boxed = Box::from_raw(ptr);
        // 気をつれないとたとえば下記のように2つ目のBoxも作れてしまう。
        // これはRustの仮定を破ってしまう
        // let boxed2 = Box::from_raw(ptr);
    }
}
