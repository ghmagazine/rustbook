use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());  // 桜文鳥
    v.push("Budgerigar".to_string());  // セキセイインコ
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));
}
