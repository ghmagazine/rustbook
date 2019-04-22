fn main() {
    #[allow(unused_variables)]
    let a1 = [false, true, false];     // [bool; 3]型
    let a2 = [0.0, -1.0, 1.0, 0.5];    // [f64; 4]型

    // `len()`で配列の長さを得られる
    assert_eq!(a2.len(), 4);

    // 長さ100の配列を作り、全要素を0i32で初期化する
    // （要素の型はCopyトレイトを実装していなければならない）
    let a3 = [0; 100];                 // [i32; 100]型
    assert_eq!(a3.len(), 100);

    // 配列は入れ子にできる
    #[allow(unused_variables)]
    let a4 = [['a', 'b'], ['c', 'd']]; // [[char; 2]; 2]型。2次元配列

    // 配列は同じ型の要素の並び。異なる型の要素は持てない
    // let a5 = [false, 'a'];
    //   → コンパイルエラー E0308：型の不一致。boolでなくcharがある

    // 配列の長さは実行時に指定できない
    let size = 100;
    // let a1 = [0; size];
    //   → コンパイルエラー E0435：
    //     コンパイル時定数が要求される場所に定数でない値がある

    // ベクタなら実行時に長さを指定できる
    let mut v1 = vec![0; size];    // Vec<i32>型
    assert_eq!(v1.len(), 100);     // 長さは100

    // ベクタには要素を追加したり、削除したりできる。
    v1.push(1);                    // ベクタの最後尾に要素を追加する
    assert_eq!(v1.len(), 101);     // 長さは101になる
    assert_eq!(v1.pop(), Some(1)); // ベクタの最後尾から要素を取り除く
    assert_eq!(v1.len(), 100);     // 長さは100に戻る

    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');

    let mut array2 = [0, 1, 2];
    array2[1] = 10;
    assert_eq!(array2, [0, 10, 2]);

    // インデックスは定数でなくてもかまわない
    let mut index = 0;
    assert_eq!(array2[index], 0);
    index += 1;
    assert_eq!(array2[index], 10);

    let array3 = [0, 1];
    // array3[2];
    // → コンパイルエラー：インデックスが範囲外。長さは2だがインデックスが2である
    #[allow(unused_variables)]
    let index = 2;
    // array3[index];
    // → コンパイルエラーにはならずにパニックする


    assert_eq!(array3.get(1), Some(&1));  // get()はインデックスが範囲内の時はSome(&値)を返す
    assert_eq!(array3.get(2), None);      // さもなければNoneを返す

    let array4 = ['a'; 50];    // 長さ50

    // iter()で要素が不変のイテレータを作成
    for ch in array4.iter() {
        print!("{},", *ch);
    }

    let mut array5 = [1; 50];  // 長さ50

    // iter_mut()で要素が可変のイテレータを作成
    for n in array5.iter_mut() {
        *n *= 2;
    }
}
