use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserResponse {
    pub data: CreateUserWrapper,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateUserWrapper {
    pub create_user: User,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
// To match GraphQL response field camelCase,
// we use `serde::rename_all` to change our Rust struct field into camelCae.
pub struct User {
    pub id: i32,
    pub name: String,
    pub full_name: Option<String>,
}
