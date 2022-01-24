use juniper::GraphQLObject;

#[derive(Debug, GraphQLObject)]
pub struct Health {
    pub status: String,
}
