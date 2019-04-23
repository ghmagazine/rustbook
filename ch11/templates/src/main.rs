use actix_web::{error, http, server, App, HttpResponse, Path, State};
use serde_derive::*;
use tera::{compile_templates, Context, Tera};

struct AppState {
    template: Tera,
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn hello_template(
    app: State<AppState>,
    path: Path<HelloPath>,
) -> Result<HttpResponse, error::Error> {
    // テンプレートに渡す値を作る
    let mut context = Context::new();
    context.insert("name", &path.name);
    // app.templateでテンプレートが取得出来る
    let body = app
        .template
        // `Tera::render` で指定したテンプレートをレンダリングできる
        .render("index.html.tera", &context)
        // レンダリングに失敗したらサーバ内部のエラーとして扱う
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;
    // レンダリング結果をレスポンスとしてステータス200 OKで返す
    Ok(HttpResponse::Ok().body(body))
}

fn main() {
    server::new(|| {
        // AppStateを準備する
        let app = AppState {
            // compile_templates! でテンプレートを一括でコンパイルできる
            template: compile_templates!("templates/**/*"),
        };
        // with_stateでアプリケーションデータとして保持
        App::with_state(app).route("/{name}", http::Method::GET, hello_template)
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}
