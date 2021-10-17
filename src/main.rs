#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use dotenv::dotenv;

#[rocket::main]
async fn main() {
    dotenv().ok();

    let rocket = zelda::rocket().await;
    if let Err(err) = rocket.launch().await {
        println!("Rocket 启动错误: {}", err);
    }
}
