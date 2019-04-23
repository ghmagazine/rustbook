trait As<T> {
    fn cast(self) -> T;
}

// 実装をジェネリックにせずに個別の型に対して実装する
impl As<u64> for u8 {
    fn cast(self) -> u64 {
        self as u64
    }
}

// 同じ`As`を`u8`に実装しているが、パラメータが異なるので問題ない
impl As<u32> for u8 {
    fn cast(self) -> u32 {
        self as u32
    }
}

fn main() {
    // トレイト実装で指定した型はcastに指定できる
    let _one_u32: u32 = 1.cast();
    let _one_u32: u64 = 1.cast();
    // `i8` は指定していないのでこれはエラー
    // error[E0277]: the trait bound `{integer}: As<i8>` is not satisfied
    // let _one_u32: i8 = 1.cast();
}
