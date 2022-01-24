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

use anyhow::Context;
use graphql::{Mutation, Query, Schema};
use juniper::EmptySubscription;
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
        Mutation,
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
