use actix_web::{App, HttpServer};
// actix_filesでスタティックファイルをサポート
use actix_files::Files;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // `Files`を用いて現在のディレクトリ下のファイルを提供する
            Files::new("/", ".")
                // ディレクトリにアクセスしたらそのディレクトリ一覧を表示する
                .show_files_listing(),
        )
    })
    .bind("localhost:3000")?
    .run()
    .await
}
