#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use rocket::{catchers, Build, Rocket};

pub mod catchers;
pub mod models;

#[must_use]
pub fn rocket() -> Rocket<Build> {
    rocket::build().register("/", catchers![catchers::not_found])
}
