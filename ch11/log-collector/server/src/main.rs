#[macro_use]
extern crate diesel;
use actix_service::ServiceFactory;
use actix_web::body::Body;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::Error;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod model;
mod schema;

// アプリケーションで持ち回る状態
#[derive(Clone)]
pub struct Server {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Server { pool }
    }
}

// ルーティングなどを書く。
// ちょっと返り値の型がゴツい。
pub fn app(
    server: Server,
) -> App<
    impl ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<Body>,
        Error = Error,
        InitError = (),
    >,
    Body,
> {
    use crate::handlers::*;

    App::new()
        .data(server)
        .service(
            web::resource("/logs")
                .route(web::post().to(handle_post_logs))
                .route(web::get().to(handle_get_logs)),
        )
        .service(
            web::resource("/csv")
                .route(web::post().to(handle_post_csv))
                .route(web::get().to(handle_get_csv)),
        )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 環境変数でログレベルを設定する
    env_logger::init();

    let server = Server::new();
    // サーバデータ(= コネクションプール)をcloneしているので
    // スレッド間で共通のコネクションプールを使うことになる。
    HttpServer::new(move || app(server.clone()))
        .bind("localhost:3000")?
        .run()
        .await
}
