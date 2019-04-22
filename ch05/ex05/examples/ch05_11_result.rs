fn main() {
    // str::parse()は文字列を指定した型（ここではi32型）に変換する
    assert_eq!("10".parse::<i32>(), Ok(10));    // 変換できたらOK(値)が返される
    let res0 = "a".parse::<i32>();              // 変換できなかったら`Err(エラーを表す値)`が返される
    assert!(res0.is_err());
    println!("{:?}", res0); // → Err(ParseIntError { kind: InvalidDigit })

    // 複数のResult値を扱うときは?演算子が便利
    // Ok(値)なら値を取り出し、Err(エラーを表す値)ならこの関数からリターンする
    fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
        let s0 = s0.parse::<i32>()?;
        let s1 = s1.parse::<i32>()?;
        Ok(s0 + s1)
    }

    assert_eq!(add0("3", "127"), Ok(3 + 127));
    assert!(add0("3", "abc").is_err());

    // map_err()コンビネータを使うとErr(エラーを表す値)のときに別のエラーに変換できる
    fn add1(s0: &str, s1: &str) -> Result<i32, String> {
        let s0 = s0.parse::<i32>().map_err(|_e| "s0が整数ではありません")?;
        let s1 = s1.parse::<i32>().map_err(|_e| "s1が整数ではありません")?;
        Ok(s0 + s1)
    }

    assert_eq!(add1("3", "abc"), Err("s1が整数ではありません".to_string()));
}
