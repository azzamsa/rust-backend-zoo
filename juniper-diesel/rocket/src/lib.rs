mod graphql;
pub mod health; // `public` for integration test purpose
pub mod logger;
mod routes;

use juniper::{EmptyMutation, EmptySubscription};
use rocket::{Build, Rocket};

use graphql::{Context, Query, Schema};

#[must_use]
pub fn rocket() -> Rocket<Build> {
    Rocket::build()
        .manage(Context {})
        .manage(Schema::new(
            Query,
            EmptyMutation::<Context>::new(),
            EmptySubscription::<Context>::new(),
        ))
        .mount(
            "/",
            rocket::routes![
                routes::graphql_playground,
                routes::get_graphql_handler,
                routes::post_graphql_handler
            ],
        )
}
