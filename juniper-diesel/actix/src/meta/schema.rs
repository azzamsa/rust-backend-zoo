use juniper::GraphQLObject;

#[derive(Debug, GraphQLObject)]
pub struct Meta {
    pub build: String,
    pub version: String,
}
