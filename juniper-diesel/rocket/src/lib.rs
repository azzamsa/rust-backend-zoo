pub mod logger;

use juniper::{EmptyMutation, EmptySubscription, RootNode};
use rocket::{response::content, Build, Rocket, State};

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub struct Context;
pub struct Query;

impl juniper::Context for Context {}

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub fn hello() -> &str {
        "Hello, world!"
    }
}

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(schema, context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(schema, context)
}

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
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
