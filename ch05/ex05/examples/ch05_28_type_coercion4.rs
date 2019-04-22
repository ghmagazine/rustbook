fn f1(p: &[i32])     -> i32 { p[0] }
fn f2(p: Box<[i32]>) -> i32 { p[0] }

fn main() {
    let a1 = [1, 2, 3, 4];
    assert_eq!(1, f1(&a1));           // &[i32; 4] → &[i32]
    assert_eq!(1, f2(Box::new(a1)));  // Box<[i32; 4]> → Box<[i32]>

    // dの型をDebugトレイトのトレイトオブジェクトに指定する
    let mut _d: Box<std::fmt::Debug>;

    // Debugトレイトを実装する型はトレイトオブジェクトへ型強制できる
    _d = Box::new([1, 2]);   // Box<[i32; 2]>  → Box<dyn Debug>
    _d = Box::new(Some(1));  // Box<Some<i32>> → Box<dyn Debug>
}
