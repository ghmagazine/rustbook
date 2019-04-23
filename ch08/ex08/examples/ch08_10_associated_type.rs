use std::str::FromStr;

trait Server {
    // `type 型名`で関連型を宣言できる
    type Response;
    // あるいは`type 型名: トレイト境界` で境界を設定することもできる
    type Request: FromStr;

    // 関連型を参照するには`Self::型名`でアクセスする
    fn handle(&self, req: Self::Request) -> Self::Response;
}

struct EchoServer;
// `Server` トレイトを実装する
impl Server for EchoServer {
    // トップレベルと同じように`type 型名 = 型名`で定義できる
    type Response = String;
    // トレイト境界のついた型も同じように定義できる
    // トレイト境界を満たさない型を書くとコンパイルエラーになる
    type Request = String;

    // 関連型を参照するには`Self::型名`でアクセスする
    fn handle(&self, req: Self::Request) -> Self::Response {
        req
    }
}

// // `S::Response`のようにServerの関連型を参照できる
// // 関連型については特別指定しなければ任意の関連型を受け付ける
// fn handle<S: Server>(server: S, req: &str) -> S::Response {
//     // 関連型にトレイト境界がついているのでトレイトの関数を呼び出すこともできる
//     let req = S::Request::from_str(&req).unwrap();
//     server.handle(req)
// }

// あるいは、関連型が特定の型を持っていることを指定したければ、`トレイト名<関連型名 = 型>`のように指定できる
// この場合RequestにStringを持つServerの実装しか受け付けない
fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
    server.handle(req.to_string())
}

fn main() {
    let server = EchoServer;
    assert_eq!(handle(server, "Hello"), "Hello");
}

// どこかに定義されたToJsonトレイトを実装して欲しいとする
trait ToJson {}

trait Server2 {
    type Request: FromStr;
    // 返り値にトレイトを書くことはできない
    fn handle(&self, req: Self::Request) -> ToJson;
}

trait Foo<T> {}
trait Bar {
    type T;
}

#[allow(dead_code)]
struct Baz;

impl Foo<i32> for Baz {}
impl Foo<char> for Baz {}
// impl Bar for Baz {
//     type T = i32;
// }
// impl Bar for Baz {
//     type T = char;
// }
