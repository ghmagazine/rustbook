fn main() {
    use std::collections::HashMap;

    let mut m1 = HashMap::new();        // またはwith_capacity(要素数)

    // 要素を2つ追加する
    m1.insert("a", 1);                  // キー："a"、バリュー：1
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);            // 要素数は2

    // キーに対応する値を取り出す
    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);      // キーが存在しないのでNone

    // "d"が存在するならその値への参照を得る。存在しないなら"d"に対して0を登録してから参照を返す
    let d = m1.entry("d").or_insert(0);
    *d += 7;

    assert_eq!(m1.get("d"), Some(&7));

    #[allow(unused_variables)]
    let m2 = vec![("a", 1), ("b", 3)].into_iter().collect::<HashMap<_, _>>();
}
