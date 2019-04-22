#[derive(Debug, PartialEq)]
struct UniqueValue;
// 以下の形式も可能
// struct UniqueValue {}
// struct UniqueValue();

fn main() {
    // フィールドがないので作れる値は1つのみ
    let uv1 = UniqueValue;
    let uv2 = UniqueValue;
    assert_eq!(uv1, uv2);
}
