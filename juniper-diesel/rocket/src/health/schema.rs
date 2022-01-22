use juniper::GraphQLObject;

// integration test needs `Deseralize` for its convenience.
#[derive(Debug, GraphQLObject)]
pub struct Health {
    pub status: String,
}
