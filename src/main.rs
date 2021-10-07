#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use dotenv::dotenv;

#[rocket::main]
async fn main() {
    dotenv().ok();

    if let Err(err) = zelda::rocket().launch().await {
        println!("Rocket Err: {}", err);
    }
}
