use actix_web::{error, web, App, HttpResponse, HttpServer};
use serde_derive::*;
use tera::{compile_templates, Context, Tera};

struct AppState {
    template: Tera,
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

async fn hello_template(
    app: web::Data<AppState>,
    path: web::Path<HelloPath>,
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // AppStateを準備する
        let app = AppState {
            // compile_templates! でテンプレートを一括でコンパイルできる
            template: compile_templates!("templates/**/*"),
        };
        // App::dataでアプリケーションデータとして保持
        App::new()
            .data(app)
            .route("/{name}", web::get().to(hello_template))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
