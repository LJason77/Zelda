use bson::{doc, from_document};
use mongodb::Database;
use rocket::{
    futures::TryStreamExt,
    get,
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
    State,
};

use crate::models::{Results, Tip};

/// 小贴士
#[get("/tips")]
pub async fn tips(db: &State<Database>) -> Custom<Value> {
    let mut result = Results::<Tip>::default();
    let collection = db.collection::<Tip>("tips");

    let pipeline = vec![doc! {"$sample": {"size": 1}}];
    let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
    while let Some(tip) = cursor.try_next().await.unwrap() {
        result.data = Some(from_document::<Tip>(tip).unwrap());
    }

    Custom(Status::Ok, json!(result))
}

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
