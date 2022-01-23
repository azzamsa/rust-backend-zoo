use juniper::GraphQLObject;
use serde::Deserialize;

#[derive(Debug, GraphQLObject, Deserialize)]
pub struct Meta {
    pub build: String,
    pub version: String,
}
