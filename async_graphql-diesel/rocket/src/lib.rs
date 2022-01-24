// db module is public to allow reusability in integration test
pub mod db;
mod health;
pub mod logger;
mod meta;
mod routes;
mod user;

use std::env;

#[macro_use]
extern crate diesel;

use anyhow::Context;
use async_graphql::{EmptySubscription, Schema};
use rocket::{Build, Rocket};

use routes::{Mutation, Query};

pub fn get_env(name: &str) -> anyhow::Result<String> {
    env::var(&name).context(format!("`{}` is not set", &name))
}

#[must_use]
pub fn rocket() -> Rocket<Build> {
    let db_pool = db::get_pool().expect("failed to get db pool");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_pool)
        .finish();

    Rocket::build().manage(schema).mount(
        "/",
        rocket::routes![
            routes::graphql_playground,
            routes::graphql_query,
            routes::graphql_request,
        ],
    )
}
