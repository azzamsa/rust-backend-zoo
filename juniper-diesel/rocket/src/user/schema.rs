use juniper::{GraphQLInputObject, GraphQLObject};

use crate::db::schema::user_ as user;

#[derive(Debug, GraphQLObject, diesel::Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug, GraphQLInputObject, diesel::Insertable)]
// Diesel assumes that the table name is the plural, snake-case form of the struct name.
// Otherwise, it yells an error. We need to specify the table name explicitly.
#[table_name = "user"]
pub struct CreateUserInput {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug, GraphQLInputObject, diesel::AsChangeset)]
#[table_name = "user"]
pub struct UpdateUserInput {
    /// The ID of the User to modify.
    pub id: i32,
    /// The name for the User.
    pub name: String,
    /// The full name for the User.
    pub full_name: Option<String>,
}
