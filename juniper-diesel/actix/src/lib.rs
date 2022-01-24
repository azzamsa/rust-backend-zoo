// db module is public to allow reusability in integration test
pub mod db;
mod graphql;
mod health;
pub mod logger;
mod meta;
mod routes;
mod user;

use std::env;

#[macro_use]
extern crate diesel;

// pub fn get_env(name: &str) -> anyhow::Result<String> {
//     env::var(&name).context(format!("`{}` is not set", &name))
// }
