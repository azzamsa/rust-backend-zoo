mod hello;
pub mod logger;

use rocket::{Build, Rocket};

#[must_use]
pub fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", rocket::routes![hello::hello])
}
