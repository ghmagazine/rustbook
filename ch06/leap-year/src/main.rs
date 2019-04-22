// 第6章 基本構文
// 入力された年がうるう年かどうかを判断するプログラム

// `std::io` 名前空間を `io` としてインポート
use std::io;
// `std::io::Write` トレイトを使う
use std::io::Write;

// エントリポイントとなる関数
fn main() {
    let mut year = String::new();
    print!("Please input a year to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year.", year);
    }
}

// うるう年の場合は `true` 、平年の場合は `false` を返す関数
fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}
