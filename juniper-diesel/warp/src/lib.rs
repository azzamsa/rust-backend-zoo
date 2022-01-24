// db module is public to allow reusability in integration test
pub mod db;
mod graphql;
mod health;
pub mod logger;
mod meta;
mod user;

use std::env;

#[macro_use]
extern crate diesel;

use anyhow::Context;
use graphql::{Mutation, Query, Schema};
use juniper::EmptySubscription;
use warp::Filter;

pub fn get_env(name: &str) -> anyhow::Result<String> {
    env::var(&name).context(format!("`{}` is not set", &name))
}

// Surprised by the type of return value
// https://github.com/seanmonstar/warp/blob/3ff2eaf41eb5ac9321620e5a6434d5b5ec6f313f/examples/todos.rs#L62
pub fn server() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let context = warp::any().map(move || graphql::Context {
        pool: db::get_pool().expect("failed to get database pool"),
    });
    let schema = Schema::new(
        Query,
        Mutation,
        EmptySubscription::<graphql::Context>::new(),
    );
    let graphql_filter = juniper_warp::make_graphql_filter(schema, context.boxed());

    let log = warp::log("warp_server");

    warp::path!("graphql")
        .and(warp::get())
        .and(juniper_warp::graphiql_filter("/graphql", None))
        .or(warp::path("graphql").and(graphql_filter))
        .with(log)
}
