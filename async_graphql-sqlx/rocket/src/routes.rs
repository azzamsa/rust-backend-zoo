use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, MergedObject, Schema,
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{response::content, State};

use crate::health::resolver::HealthQuery;
use crate::meta::resolver::MetaQuery;
use crate::user::resolver::{UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(HealthQuery, MetaQuery, UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[rocket::get("/")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<AppSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json", rank = 1)]
pub async fn graphql_request(
    schema: &State<AppSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema).await
}
