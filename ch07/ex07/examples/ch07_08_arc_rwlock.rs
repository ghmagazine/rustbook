use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

// ?演算子を使うためmain関数からResult型を返すようにする
fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["柴", "トイプードル"].iter().cloned().collect();
    // HashSetを可変にするためにRwLockで包み、スレッド間で共有するためにArcで包む
    let dogs = Arc::new(RwLock::new(dogs));  // Arc<RwLock<HashSet<&'static str>>>型

    // PoisonErrorをStringに変換するのに便利な関数を定義しておく
    fn stringify(x: impl ToString) -> String { x.to_string() }

    {
        let ds = dogs.read().map_err(stringify)?;    // readロックを取得する
        assert!(ds.contains("柴")); 
        assert!(ds.contains("トイプードル"));
    }   // dsがスコープを外れロックが解除される

    // writeロックを取得しHashSetに要素を追加する
    dogs.write().map_err(stringify)?.insert("ブル・テリア");

    let dogs1 = Arc::clone(&dogs);
    std::thread::spawn(move ||
        // 別のスレッドでwriteロックを取得しHashSetに要素を追加する
        dogs1.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify)
    ).join().expect("Thread error")?;   // スレッドの終了を待つ

    // このスレッドと別スレッドの両方で追加した要素が見える
    assert!(dogs.read().map_err(stringify)?.contains("ブル・テリア"));
    assert!(dogs.read().map_err(stringify)?.contains("コーギー"));
    Ok(())
}
