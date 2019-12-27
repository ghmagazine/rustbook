use crate::db;
use crate::Server;
use actix_multipart::Multipart;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, HttpResponse, Result};
use diesel::pg::PgConnection;
use futures::prelude::*;
use itertools::Itertools;
use log::debug;
use std::io::{self, prelude::*};

// ハンドラで大きく変わったのは3箇所。
// 1. ハンドラが `async` に
// 2. エラー型がFailureの `Error` から独自の `Error` に
//   + 各所に `.map_err` を挟む必要が出た
// 3. マルチパートの処理をactix-multipartで書き直した
//
// マルチパートはActix-Web 0.7の頃とあまりAPIに変更はないが、`async` / `await` が入ったことにより
// 泥臭い処理をしててもある程度読みやすくなったのでラッパを作らずにそのままのAPIを使うことにした。
//
// 全体的な注意としては `async` ハンドラの中でファイルIOやDB接続などのブロッキング処理をしている点。
// ブロッキング処理をするとスレッドを止めてしまうので、
// 非同期処理のメリットである「1スレッドでいくつもの処理をこなせる」を完全に殺してしまう。
// とはいえCSVリーダーやテンポラリファイル、DB接続など外部のクレートはどうしようもない。
// このあたりは非同期のエコシステムが成熟するのを待つことになる。
//
// どうしても非同期処理の中で同期処理を扱いときは別スレッドを立てて同期処理を逃がすのが定石。
// Actixも `actix_rt::Arbiter::exec` というAPIを用意している。
// Arbiter (=決定者) はスレッド+イベントループを表わすエンティティで、Arbiterを作ればスレッドが作られるし、
// その上でFutureを走らせることができる。
//
// 今回はActix-Web 0.7の頃から非同期処理の中で同期処理を書いてしまっていたのもあってArbiterは使わずに同期処理をしてしまった。
// 他には扱っているデータがスレッドアンセーフなので別スレッドに送ろうとすると面倒事が多いなどの理由もある。
// 現実的に問題を解決したいならActixのアクターシステムを使うなどの手段もある。
// 1. DBはDB用のActorを立てて、SQLを発行する関数は全てそこで実行してしまう。
// 2. マルチパートやCSVの扱いは一旦ファイルに保存するまでをリクエストハンドラのArbiter内でやり、CSVファイルをデコードしてDBに保存するまでを別Actorにする
//

async fn load_file(conn: &PgConnection, file: impl Read) -> Result<usize> {
    use crate::model::NewLog;

    let mut ret = 0;

    // CSVファイルが渡される`csv::Reader`を用いてapi::Logにデコードしていく
    let in_csv = io::BufReader::new(file);
    let in_log = csv::Reader::from_reader(in_csv).into_deserialize::<::api::Log>();
    // Itertoolsのchunksを用いて1000件ずつ処理する
    // blocking
    for logs in &in_log.chunks(1000) {
        let logs = logs
            // Logとしてパースできた行だけ集める
            .filter_map(Result::ok)
            .map(|log| NewLog {
                user_agent: log.user_agent,
                response_time: log.response_time,
                timestamp: log.timestamp.naive_utc(),
            })
            .collect_vec();

        // blocking
        let inserted = db::insert_logs(conn, &logs).map_err(ErrorInternalServerError)?;
        ret += inserted.len();
    }

    Ok(ret)
}

/// `POST /csv`のハンドラ
pub async fn handle_post_csv(
    server: web::Data<Server>,
    // actix-multipartを使ってマルチパートのリクエストを受け取る
    mut multipart: Multipart,
) -> Result<HttpResponse> {
    let conn = server.pool.get().map_err(ErrorInternalServerError)?;
    let mut total_size = 0;

    // multipartsは `Stream` になっている。
    // `Future` は1度待つと1つの値が得られるが、 `Stream` は何度も待って何度も値が得られる。
    // 非同期版のイテレータのような存在。
    //
    // 生の `Stream` をまともに扱う手段はないが、futuresのおかげで `.next()` メソッドが生えてきている。
    // これでイテレータのように扱える。
    while let Some(field) = multipart.next().await {
        // データ(= ここではファイル)を取り出す度に `await` しているので並列性はない。
        // ただし他のリクエストハンドラが動けるのでサーバ全体のパフォーマンスは上がる。

        let mut field = field.map_err(ErrorBadRequest)?;
        // text/csvでなければスキップ
        if field.content_type().as_ref() != "text/csv" {
            continue;
        }
        // ファイルでなければスキップ
        if !field
            .content_disposition()
            .map(|c| c.is_attachment())
            .unwrap_or(false)
        {
            continue;
        }
        // 一旦データをファイルに書き出す
        // blocking
        let mut file = io::BufWriter::new(tempfile::tempfile().map_err(ErrorInternalServerError)?);
        // データが分割されて送られて来るのでチマチマ受け取ってファイルに書く。
        while let Some(data) = field.next().await {
            let data = data.map_err(ErrorInternalServerError)?;
            // blocking
            file.write(&data).map_err(ErrorInternalServerError)?;
        }
        // 書き出したデータをDBにロードする
        // blocking
        let file = file.into_inner().map_err(ErrorInternalServerError)?;
        // blocking
        total_size = load_file(&conn, file).await?;
    }
    Ok(HttpResponse::Ok().json(api::csv::post::Response(total_size)))
}

/// `POST /logs`のハンドラ
pub async fn handle_post_logs(
    server: web::Data<Server>,
    log: web::Json<api::logs::post::Request>,
) -> Result<HttpResponse> {
    use crate::model::NewLog;
    use chrono::Utc;

    let log = NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log.timestamp.unwrap_or_else(|| Utc::now()).naive_utc(),
    };
    let conn = server.pool.get().map_err(ErrorInternalServerError)?;
    // blocking
    db::insert_log(&conn, &log).map_err(ErrorInternalServerError)?;

    debug!("received log: {:?}", log);

    Ok(HttpResponse::Accepted().finish())
}

/// `GET /logs`のハンドラ
pub async fn handle_get_logs(
    server: web::Data<Server>,
    range: web::Query<api::logs::get::Query>,
) -> Result<HttpResponse> {
    use chrono::{DateTime, Utc};

    let conn = server.pool.get().map_err(ErrorInternalServerError)?;
    // blocking
    let logs = db::logs(&conn, range.from, range.until).map_err(ErrorInternalServerError)?;
    let logs = logs
        .into_iter()
        .map(|log| api::Log {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: DateTime::from_utc(log.timestamp, Utc),
        })
        .collect();

    Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

/// `GET /csv`のハンドラ
pub async fn handle_get_csv(
    server: web::Data<Server>,
    range: web::Query<api::csv::get::Query>,
) -> Result<HttpResponse> {
    use chrono::{DateTime, Utc};

    let conn = server.pool.get().map_err(ErrorInternalServerError)?;
    // blocking
    let logs = db::logs(&conn, range.from, range.until).map_err(ErrorInternalServerError)?;
    let v = Vec::new();
    let mut w = csv::Writer::from_writer(v);

    for log in logs.into_iter().map(|log| ::api::Log {
        user_agent: log.user_agent,
        response_time: log.response_time,
        timestamp: DateTime::from_utc(log.timestamp, Utc),
    }) {
        // ブロッキングに見えるが実体は `Vec` なのでブロックしない
        w.serialize(log).map_err(ErrorInternalServerError)?;
    }

    let csv = w.into_inner().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(csv))
}
