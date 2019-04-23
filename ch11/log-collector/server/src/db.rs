use crate::model::*;
use chrono::{DateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::QueryResult;

pub fn insert_log(cn: &PgConnection, log: &NewLog) -> QueryResult<i64> {
    use crate::schema::logs::dsl;
    insert_into(dsl::logs)
        .values(log)
        .returning(dsl::id)
        .get_result(cn)
}

pub fn insert_logs(cn: &PgConnection, logs: &[NewLog]) -> QueryResult<Vec<i64>> {
    use crate::schema::logs::dsl;
    insert_into(dsl::logs)
        .values(logs)
        .returning(dsl::id)
        .load(cn)
}

pub fn logs(
    cn: &PgConnection,
    from: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
) -> QueryResult<Vec<Log>> {
    use crate::schema::logs::dsl;

    // 型エラーを防ぐためにinto_boxedを呼んでおく
    let mut query = dsl::logs.into_boxed();
    if let Some(from) = from {
        query = query.filter(dsl::timestamp.ge(from.naive_utc()))
    }
    if let Some(until) = until {
        query = query.filter(dsl::timestamp.lt(until.naive_utc()))
    }
    query.order(dsl::timestamp.asc()).load(cn)
}
