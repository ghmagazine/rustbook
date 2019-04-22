// この関数は引数として&str型の名前を取り、&str型の"Hello, 名前!"を返す
// fn f1(name: &str) -> &str {
//     let s = format!("Hello, {}!", name); // format!はStringを作る
//     &s   // Stringから&strを作成し、戻り値として返す
//     // → コンパイルエラー：`s` does not live long enough.（sの生存期間が不十分）
// }

// この関数は引数として&str型の名前を取り、String型の"Hello, 名前!"を返す
fn f1(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    f1("ken");
}
