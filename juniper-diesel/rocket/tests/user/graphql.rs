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

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", argument_struct = "ReadUserArguments")]
    pub struct UserQuery {
        #[arguments(id = &args.id)]
        pub user: User,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct ReadUserArguments {
        pub id: i32,
    }
}

#[cynic::schema_for_derives(file = "schema.graphql", module = "schema")]
pub mod mutations {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "CreateUserInput")]
    pub struct UserMutation {
        #[arguments(input =
              CreateUserInput {
                 name: args.name.clone(),
                 full_name: args.full_name.clone(),
            }
        )]
        pub create_user: User,
    }

    #[derive(cynic::InputObject, cynic::FragmentArguments, Debug)]
    pub struct CreateUserInput {
        pub name: String,
        pub full_name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub full_name: Option<String>,
    }
}
