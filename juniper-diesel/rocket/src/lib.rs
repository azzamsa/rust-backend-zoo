mod db;
mod graphql;
pub mod health; // `public` for integration test purpose
pub mod logger;
pub mod meta;
mod routes;
pub mod user;

use std::env;

#[macro_use]
extern crate diesel;

use anyhow::Context;
use graphql::{Query, Schema};
use juniper::{EmptyMutation, EmptySubscription};
use rocket::{Build, Rocket};

pub fn get_env(name: &str) -> anyhow::Result<String> {
    env::var(&name).context(format!("`{}` is not set", &name))
}

#[must_use]
pub fn rocket() -> Rocket<Build> {
    let context = graphql::Context {
        pool: db::get_pool().expect("failed to get database pool"),
    };
    let schema = Schema::new(
        Query,
        EmptyMutation::<graphql::Context>::new(),
        EmptySubscription::<graphql::Context>::new(),
    );
    Rocket::build().manage(context).manage(schema).mount(
        "/",
        rocket::routes![
            routes::graphql_playground,
            routes::get_graphql_handler,
            routes::post_graphql_handler
        ],
    )
}
