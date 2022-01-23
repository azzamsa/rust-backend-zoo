use juniper::GraphQLObject;
use serde::Deserialize;

// integration test needs `Deseralize` for its convenience.
#[derive(Debug, GraphQLObject, Deserialize)]
pub struct Health {
    pub status: String,
}
