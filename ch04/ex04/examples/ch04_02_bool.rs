#[allow(unused_variables)]
fn main() {
    let b1 = true;
    let b2 = !b1;       // false、否定

    let n1 = 8;
    let n2 = 12;
    let b3 = n1 >= 10;  // false
    let b4 = n2 >= 10;  // true
    let b5 = b3 && b4;  // false、ショートサーキット論理積
    let b6 = b3 || b4;  // true、ショートサーキット論理和

    assert_eq!(std::mem::size_of::<bool>(), 1);  // サイズは1バイト
}
