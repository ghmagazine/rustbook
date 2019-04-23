use std::io::Cursor;
use bicycle_book_wordcount::{count, CountOption};

#[macro_use]
mod utils;

// 以下にテストを書く

#[test]
fn line_count_works() {
    let input = Cursor::new(
        r#"Tokyo, Japan
Kyoto, Japan
Tokyo, Japan
Shanghai, China
"#,
    );

    let freq = count(input, CountOption::Line);

    assert_map!(freq, {
        "Tokyo, Japan" => 2,
        "Kyoto, Japan" => 1,
        "Shanghai, China" => 1
    });
}

#[test]
fn line_count_lfcr() {
    let input = Cursor::new("aa\r\nbb\r\ncc\r\nbb");

    let freq = count(input, CountOption::Line);

    assert_map!(freq, {
        "aa" => 1,
        "bb" => 2,
        "cc" => 1
    });
}
