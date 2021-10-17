#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use mongodb::{Client, Database};
use rocket::{catchers, routes, Build, Rocket};

use api::index;

pub mod api;
pub mod catchers;
pub mod models;

/// 初始化 mongodb 数据库
pub async fn init_mongo(name: &str) -> Database {
    let database_url =
        dotenv::var("MONGO_URL").unwrap_or(format!("mongodb://root:root@127.0.0.1:27017/{}", name));
    let client = Client::with_uri_str(&database_url)
        .await
        .expect("连接数据库失败：");
    client.database(name)
}

pub async fn rocket() -> Rocket<Build> {
    let db = init_mongo("zelda").await;

    rocket::build()
        .manage(db)
        .mount("/", routes![index::health_check])
        // 错误处理
        .register("/", catchers![catchers::not_found])
}
