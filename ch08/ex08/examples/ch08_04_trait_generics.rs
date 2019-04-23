trait Init<T> {
    fn init(t: T) -> Self;
}

impl<T> Init<T> for Box<T> {
    // 内部では`T`でパラメータの型を参照する
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

fn main() {
    // ジェネリクスが推論可能なら省略できる
    let _data = Box::init("foo");
    // トレイトのジェネリク型を明示するには`型名::<型>`と書く
    let _data = Box::<f32>::init(0.1);
    let _data: Box<f32> = Init::init(0.1);
    let _data: Box<_> = Init::<f32>::init(0.1);
}
