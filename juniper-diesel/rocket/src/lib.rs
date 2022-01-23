mod db;
mod graphql;
pub mod health; // `public` for integration test purpose
pub mod logger;
pub mod meta;
mod routes;
pub mod user;

#[macro_use]
extern crate diesel;

use juniper::{EmptyMutation, EmptySubscription};
use rocket::{Build, Rocket};

use graphql::{Context, Query, Schema};

#[must_use]
pub fn rocket() -> Rocket<Build> {
    let context = Context {
        pool: db::get_pool(),
    };
    let schema = Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
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
