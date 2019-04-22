use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());     // 桜文鳥
    v.push("Budgerigar".to_string());     // セキセイインコ

    let mut iter = v.iter();

    // v.push("Hill Mynah".to_string());  // 九官鳥。コンパイルエラーになる
    // → error[E0502]: cannot borrow `v` as mutable because it is
    //   also borrowed as immutable
    // pushは可変の参照を得ようとするが、iterが生存しているので不変の参照が有効

    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
    v.push("Canary".to_string());  // カナリア。iterはもう生存していないので変更できる
}
