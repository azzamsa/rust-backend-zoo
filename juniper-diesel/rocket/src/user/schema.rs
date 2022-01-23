use diesel::Queryable;
use juniper::GraphQLObject;
use serde::Deserialize;

#[derive(Debug, GraphQLObject, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub full_name: Option<String>,
}
