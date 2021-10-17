use mongodb::Database;
use rocket::{
    get,
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
    State,
};

use crate::models::Results;

/// 健康检查
#[must_use]
#[get("/health")]
pub fn health_check(db: &State<Database>) -> Custom<Value> {
    // TODO 健康检查
    let result = Results::<'_, &str> {
        message: Some("服务器运行正常"),
        data: Some(db.name()),
        ..Results::default()
    };
    Custom(Status::Ok, json!(result))
}
