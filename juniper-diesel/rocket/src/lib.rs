mod graphql;
mod health;
pub mod logger;

use rocket::{response::content, Build, Rocket, State};

use graphql::schema::AppSchema;
use graphql::schema::Context;

#[rocket::get("/")]
fn graphql_playground() -> content::Html<String> {
    juniper_rocket_async::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<AppSchema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<AppSchema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[must_use]
pub fn rocket() -> Rocket<Build> {
    let schema = graphql::schema::build();

    rocket::build().manage(schema).mount(
        "/",
        rocket::routes![
            graphql_playground,
            get_graphql_handler,
            post_graphql_handler
        ],
    )
}
