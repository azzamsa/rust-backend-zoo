use crate::db::DbPool;
use juniper::FieldResult;

use super::model;
use super::schema::{CreateUserInput, User};

pub fn read_all(pool: &DbPool) -> FieldResult<Vec<User>> {
    let users = model::find_all(pool)?;
    Ok(users)
}
pub fn read(pool: &DbPool, id: i32) -> FieldResult<User> {
    let user = model::find(pool, id)?;
    Ok(user)
}
pub fn create(pool: &DbPool, user_input: CreateUserInput) -> FieldResult<User> {
    let user = model::create(pool, user_input)?;
    Ok(user)
}
