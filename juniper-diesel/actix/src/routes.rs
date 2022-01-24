use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};
use rocket::{response::content, State};

use crate::graphql::{Context, Schema};

pub async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}
pub async fn graphql_playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}
pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}
