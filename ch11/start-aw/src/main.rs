use actix_web::{web, App, HttpServer, Responder};

use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    // `{name}` に対応するフィールドを定義する
    name: Option<String>,
}

// extractorsを使った書き方に変更(0.7の頃とあまり変わってない)
// 関数定義が `async fn` になった。このハンドラでは `async` はほぼ飾り
async fn hello(path: web::Path<HelloPath>) -> impl Responder {
    let to = path.name.as_ref().map(|s| s.as_ref()).unwrap_or("World");
    format!("Hello {}!", to)
}

// async fn(やasyncブロック)については下記ブログ記事などを参考に。
//
// 1. [The Async Book](https://rust-lang.github.io/async-book/index.html)
// 2. [Async-await on stable Rust! | Rust Blog](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html)
// 3. [Rustのasync/awaitの特徴4つ - Qiita](https://qiita.com/qnighy/items/05c38f73ef4b9e487ced)
// 4. [Rustの非同期プログラミングをマスターする - OPTiM TECH BLOG](https://tech-blog.optim.co.jp/entry/2019/11/08/163000#Rust%E3%81%AE%E9%9D%9E%E5%90%8C%E6%9C%9F%E3%81%AE%E6%9B%B8%E3%81%8D%E6%96%B9)
// 5. [Rustのasync/awaitをスムーズに使うためのテクニック - Qiita](https://qiita.com/qnighy/items/59133e69a0ba0c6a7fef)

// mainも async fnになっている。mainにasyncを書くのはRustそのものががサポートしている訳ではなくて、
// actix_rt::mainの **アトリビュートマクロ** が上手いこと帳尻を合わせている。
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // サーバの作り方が変わった。
    // ここはあまり書き味に影響ないので「ふーん、変わったんだ」程度。
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/{name}", web::get().to(hello))
    })
    .bind("127.0.0.1:3000")?
    .run()
    // `.await` が `async` に呼応するキーワード。
    // メソッド呼び出と親和性の高い **後置キーワード** として導入された。
    .await
}

/*
↑ のコードはactix_rt::mainによって以下のように展開される。
こうすると普通のRustのmainなので実行できる。
fn main() -> std::io::Result<()> {
    actix_rt::System::new("main")
        .block_on(async move {
            HttpServer::new(|| {
                App::new()
                    .route("/", web::get().to(hello))
                    .route("/{name}", web::get().to(hello))
            })
            .bind("127.0.0.1:3000")?
            .run()
            .await
          })

}
*/
