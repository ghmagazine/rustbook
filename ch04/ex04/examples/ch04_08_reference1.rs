// 関数f1は呼び出し元の値のコピーを引数nに束縛し、1に変更する
#[allow(unused_assignments)]
fn f1(mut n: u32) {
    n = 1;
    println!("f1:       n = {}", n);
}

// 関数f2は呼び出し元の値を指すポインタを受け取り、ポインタが指す
// 場所に2を格納する
fn f2(n_ptr: &mut u32) {
    println!("f2:   n_ptr = {:p}", n_ptr);

    // *を付けると参照先にアクセスできる。これを参照外し（dereference）と呼ぶ
    *n_ptr = 2;
    println!("f2:  *n_ptr = {}", *n_ptr);
}

fn main() {
    let mut n = 0;
    println!("main:     n = {}", n);

    f1(n);
    println!("main:     n = {}", n);

    // `&mut n`でnの値を指す可変のポインタを作成する
    f2(&mut n);
    println!("main:     n = {}", n);
}
