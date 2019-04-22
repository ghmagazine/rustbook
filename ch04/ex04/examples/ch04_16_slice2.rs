fn main() {
    let mut a1 = [5, 4, 3, 2];      // 配列。[i32; 4]型
    let s1 = &mut a1[1..3];         // 可変のスライス。&mut[i32]型
    s1[0] = 6;                      // スライスの最初の要素を6にする
    s1[1] *= 10;                    // 2番目の要素を10倍する
    s1.swap(0, 1);                  // 要素を交換する
    assert_eq!(s1, [30, 6]);        // スライスの内容を確認

    // 参照元の配列の内容を確認
    assert_eq!(a1, [5, 30, 6, 2]);  // スライスを通じて配列の内容が変更された

    let a2: [i32; 0] = [];
    let s2 = &a2;                              // 不変のスライスを作成
    assert!(s2.is_empty());                    // 空のスライス
    assert_eq!(s2.len(),   0);                 // 長さは0
    assert_eq!(s2.first(), None);              // 最初の要素は存在しない

    let a3 = ["zero", "one", "two", "three", "four"];
    let s3 = &a3[1..4];                        // 不変のスライスを作成
    assert!(!s3.is_empty());                   // 空ではない
    assert_eq!(s3.len(),   3);                 // 長さは3
    assert_eq!(s3.first(), Some(&"one"));      // 最初の要素

    assert_eq!(s3[1],      "two");             // 2番目の要素
    // assert_eq!(s3[3],   "?");               // 4番目の要素。存在しないのでpanicする
    assert_eq!(s3.get(1),  Some(&"two"));      // 2番目の要素を得る別の方法
    assert_eq!(s3.get(3),  None);              // 4番目の要素。存在しないのでNone

    assert!(s3.contains(&"two"));              // "two"を要素に持つ
    assert!(s3.starts_with(&["one", "two"]));  // "one", "two"で始まる
    assert!(s3.ends_with(&["two", "three"]));  // "two", "three"で終わる

    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];

    // 一部の要素を昇順にソートする
    &mut a4[2..6].sort();
    assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

    // スライスを2つの可変スライスへ分割する
    #[allow(unused_variables)]
    let (s4a, s4b) = &mut a4.split_at_mut(5);

    // &mutを省略しても結果は同じ。型強制によって自動的にスライスが作られる
    a4[2..6].sort();
    let (s4a, s4b) = a4.split_at_mut(5);

    // 前半を逆順にする
    s4a.reverse();
    assert_eq!(s4a, &[8, 2, 0, 4, 6]);

    // 後半を昇順にソートする
    s4b.sort_unstable();
    assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);

    // sort()とsort_unstable()の違い
    // sort()は安定ソートなので同順なデータのソート前の順序がソート後も保存される
    // soft_unstable()は安定ソートではないが、一般的にsort()より高速
}
