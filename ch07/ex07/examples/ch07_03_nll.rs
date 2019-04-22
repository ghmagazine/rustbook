use std::collections::HashMap;

// この関数はHashMapにキーに対応する値がある場合はそれを変更し
// ない場合はデフォルト値を挿入する
fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    // get_mutが返す可変の参照が生存している間はmapの可変の借用が有効
    match map.get_mut(&key) {
        // valueが可変の参照に束縛される
        // つまりvalueが生存している間はmapの可変の借用が有効となる
        Some(value) => value.push_str(", world!"),
        None => {  // このブロック内ではselfの可変の借用は終了している
            // insertはselfの可変の借用をとる
            map.insert(key, Default::default());
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
}
