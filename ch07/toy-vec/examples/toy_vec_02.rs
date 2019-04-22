use toy_vec::ToyVec;

fn main() {
    let _e: Option<&String>;
    {
        let mut v = ToyVec::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());

        _e = v.get(1);  // コンパイルエラーになる
        // → error[E0597]: `v` does not live long enough

    }  // ここでvがスコープから抜け、ToyVec構造体が破棄される

    // eは解放後のメモリを参照している
    // assert_eq!(_e, Some(&"Budgerigar".to_string()));
}
