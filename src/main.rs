#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use dotenv::dotenv;
use rocket::{catchers, Build, Rocket};

mod catchers;
mod models;

fn rocket() -> Rocket<Build> {
    rocket::build().register("/", catchers![catchers::not_found])
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    if let Err(err) = rocket().launch().await {
        println!("Rocket Err: {}", err);
    }
}
