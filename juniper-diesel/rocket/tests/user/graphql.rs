mod schema {
    cynic::use_schema!("schema.graphql");
}

#[cynic::schema_for_derives(file = "schema.graphql", module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct UsersQuery {
        pub users: Vec<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub full_name: Option<String>,
    }
}
