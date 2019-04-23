use std::fmt::Display;

fn main() {
    let mut v: Vec<&dyn Display> = vec![];
    v.push(&true);
    v.push(&1i32);
}

use std::string::ToString;

// #[allow(dead_code)]
// fn stringify(t: Box<dyn ToString>) -> String {
//     t.to_string()
// }

// #[allow(dead_code)]
// fn stringify<T: ToString>(t: T) -> String {
//     t.to_string()
// }

#[allow(dead_code)]
fn stringify(t: impl ToString) -> String {
    t.to_string()
}

// 上記コードは以下のようなコードに展開される
// ※ コードはイメージです。実際のものと異なることがありま
// fn stringify(t: Box<dyn ToString>) -> String {
//     let data = t.data;
//     let to_string = t.to_string;
//     to_string(&data)
// }
