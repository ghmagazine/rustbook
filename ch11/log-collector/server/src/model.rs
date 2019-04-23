use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Insertable)]
#[table_name = "logs"]
pub struct NewLog {
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Queryable)]
pub struct Log {
    pub id: i64,
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: NaiveDateTime,
}
