// この関数は&[char]型のスライスを引数に取り、その情報を表示する
fn print_info(name: &str, sl: &[char]) {
    println!(
        "  {:9} - {}, {:?}, {:?}, {:?}",
        name,
        sl.len(),    // 長さ（バイト数）  usize型
        sl.first(),  // 最初の要素       Option<char>型
        sl[1],       // 2番目の要素      char型
        sl.last()    // 最後の要素       Option<char>型
    );
}

fn main() {
    // 配列
    let a1 = ['a', 'b', 'c', 'd'];       // 参照元のデータ。[char; 4]型
    println!("a1: {:?}", a1);

    print_info("&a1[..]",   &a1[..]);    // &[char]型。全要素のスライス
    print_info("&a1",       &a1);        // 同上
    print_info("&a1[1..3]", &a1[1..3]);  // 'b'と'c'を要素とする長さ2のスライス

    // ベクタ
    let v1 = vec!['e', 'f', 'g', 'h'];   // 参照元のデータ。Vec<char>型
    println!("\nv1: {:?}", v1);

    print_info("&v1[..]",   &v1[..]);    // &[char]型。全要素のスライス
    print_info("&v1",       &v1);        // 同上
    print_info("&v1[1..3]", &v1[1..3]);  // &[char]型。'f'と'g'を要素とする長さ2のスライス
}
