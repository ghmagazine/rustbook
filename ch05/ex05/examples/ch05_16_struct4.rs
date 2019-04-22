struct UserName(String);
struct Id(u64);
struct Timestamp(u64);
type User = (Id, UserName, Timestamp);

#[allow(dead_code)]
fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

#[allow(unused_variables)]
fn main() {
    let id = Id(400);
    let now = Timestamp(4567890123);

    // nowとidの順番を間違えるとコンパイルエラーになってくれる
    // let bad_user = new_user(UserName(String::from("kazuki")), now, id);
    // error[E0308]: mismatched types
    // expected type `Id`, found type `Timestamp`
}
