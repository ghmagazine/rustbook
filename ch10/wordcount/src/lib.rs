//! `bicycle_book_wordcount` はシンプルな文字、単語、行の出現頻度の計数機能を提供します。
//! 詳しくは[`count`](fn.count.html)関数のドキュメントを見て下さい。

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// [`count`](fn.count.html)で使うオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// 文字毎に頻度を数える
    Char,
    /// 単語毎に頻度を数える
    Word,
    /// 行毎に頻度を数える
    Line,
}

/// オプションのデフォルトは [`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// `input` から1行ずつUTF-8文字列を読み込み、頻度を数える。
///
/// 頻度を数える対象はオプションによって制御される。
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicodeの1文字毎に頻度を数える
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): 正規表現 `\w+` にマッチする単語毎に頻度を数える
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): `\n`または`\r\n` で区切られた1行毎に頻度を数える
///
/// # Examples
/// 入力中の単語の出現頻度を数える例
///
/// ```
/// use std::io::Cursor;
/// use bicycle_book_wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
///
/// # Panics
///
/// 入力がUTF-8でフォーマットされていない場合にパニックする。
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new(); // HashMap<String, usize>型

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word =>
            // 4. その行を単語で分割する
            {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    // 5. 出現した単語の出現頻度を数える
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[test]
fn word_count_works() {
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}

#[test]
fn word_count_works2() {
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("cc".to_string(), 1);
    exp.insert("dd".to_string(), 1);

    assert_eq!(count(Cursor::new("aa  cc dd"), CountOption::Word), exp);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    macro_rules! assert_map {
        ($expr: expr, {$($key: expr => $value:expr),*}) => {
            $(assert_eq!($expr[$key], $value));*
        };
    }

    #[test]
    fn word_count_works3() {
        let freqs = count(Cursor::new("aa  cc dd"), CountOption::Word);

        assert_eq!(freqs.len(), 3);
        assert_map!(freqs, {"aa" => 1, "cc" => 1, "dd" => 1});
    }

}

#[test]
#[should_panic]
fn word_count_do_not_contain_unknown_words() {
    use std::io::Cursor;

    count(
        Cursor::new([
            b'a', // a
            0xf0, 0x90, 0x80, // でたらめなバイト列
            0xe3, 0x81, 0x82, // あ
        ]),
        CountOption::Word,
    );
}
