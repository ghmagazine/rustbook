fn main() {
    #[allow(unused_variables)]
    let v1 = vec![false, true, false];    // Vec<bool>型
    let v2 = vec![0.0, -1.0, 1.0, 0.5];   // Vec<f64>型

    assert_eq!(v2.len(), 4);              // v2ベクタの長さは4

    // 長さ100のベクタを作り、全要素を0i32で初期化する
    // （要素の型はCloneトレイトを実装していなければならない）
    let v3 = vec![0; 100];                // Vec<i32>型
    assert_eq!(v3.len(), 100);

    // ベクタは入れ子にできる。子の要素数はそれぞれが異なってもかまわない
    #[allow(unused_variables)]
    let v4 = vec![vec!['a', 'b', 'c'], vec!['d']]; // Vec<Vec<char>>型

    // ベクタは同じ型の要素の並び。異なる型の要素は持てない
    // let v5 = vec![false, 'a'];
    //   → error[E0308]: mismatched types

    let mut v6 = vec!['a', 'b', 'c'];      // Vec<char>型
    v6.push('d');                          // 最後尾に値を追加
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);  // v6の現在の値

    assert_eq!(v6.pop(), Some('e'));       // 最後尾から値を取り出し
    v6.insert(1, 'f');                     // インデックス1の位置に要素を挿入
    assert_eq!(v6.remove(2), 'b');         // インデックス2の要素を削除。返り値は削除した値
    assert_eq!(v6, ['a', 'f', 'c', 'd']);  // v6の現在の値

    let mut v7 = vec!['g', 'h'];           // 別のベクタv7を作成
    v6.append(&mut v7);                    // v6の最後尾にv7の全要素を追加
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
    assert_eq!(v7, []);                    // v7は空になった（全要素がv6へ移動した）

    let a8 = ['i', 'j'];                   // 固定長配列a8を作成
    v6.extend_from_slice(&a8);             // v6の最後尾にa8の全要素を追加
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(a8, ['i', 'j']);            // a8は変更なし（a8の要素がコピーされた）
}
