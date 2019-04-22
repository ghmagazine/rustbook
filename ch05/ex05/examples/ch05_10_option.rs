fn main() {
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));  // インデックス0は配列a1の範囲内なので`Some(&値)`が返る
    assert_eq!(a1.get(4), None);        // インデックス4は範囲外なので`None`が返る

    let mut o1 = Some(10);              // Option<i32>型
    match o1 {                          // match式でバリアントが判別できる
        Some(s) => assert_eq!(s, 10),   // パターンマッチで中の値を取り出す
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {                // if let式でもバリアントの判別と値の取り出しができる
        assert_eq!(s, 20);
    }

    let mut o2 = Some(String::from("Hello"));  // Option<String>型
    assert_eq!(o2.unwrap(), "Hello");          // unwrap()でSomeの中の値が取り出せる

    // しかしunwrap()はNoneのときにpanicするので、できるだけ使わない方がいい
    o2 = None;
    // o2.unwrap();
    // → thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'

    // unwrap_or_else()ならNoneでもpanicしないので安心して使える
    // Noneのときはクロージャを実行し、Noneの代わりになる値を得る
    assert_eq!(o2.unwrap_or_else(|| String::from("o2 is none")), "o2 is none");

    // Someで包まれた値を操作するならmap()やand_then()などのコンビネータが便利

    // map()はSome(値)のときは値にクロージャを適用し、クロージャが返した値をSomeで包み直す
    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));

    // NoneならなにもせずNoneを返す
    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None);

    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
            // and_then()はSome(値)のときは値にクロージャを適用し
            // クロージャが返した値（Some(新しい値)、または、None）をそのまま返す
            .and_then(|n| if n >= 200 { Some(n) } else { None }),
        None
    );

    // インデックス0と3の両方に値があるので、それらの合計がSomeで包まれて返される
    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));

    // インデックス3がないのでNoneが返される
    assert_eq!(add_elems(&[7, 11]), None);
}

fn add_elems(s: &[i32]) -> Option<i32> {
    // 複数のOption値を扱うときは?演算子が便利
    // Some(値)なら値を取り出し、Noneならこの関数からすぐに戻る（Noneを返す）
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}
