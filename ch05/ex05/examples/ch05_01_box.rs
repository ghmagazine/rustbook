fn main() {
    let t1 = (5, "birds".to_string());  // (i32, String)型のタプル。スタックに置かれる
    let mut b1 = Box::new(t1);          // Boxポインタを作る。タプルがヒープに移動する
    (*b1).0 += 1;                       // *で参照外し
    assert_eq!(*b1, (6, "birds".to_string()));

    // Box::new()の実行後にt1にアクセスしようとするとコンパイルエラーになる
    // println!("{:?}", t1);
    //   → error[E0382]: borrow of moved value: `t1`
}
