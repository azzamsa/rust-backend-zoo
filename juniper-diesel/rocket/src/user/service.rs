use crate::db::DbPool;
use juniper::FieldResult;

use super::model;
use super::schema::User;

pub fn read_all(pool: &DbPool) -> FieldResult<Vec<User>> {
    let users = model::find_all(pool)?;
    Ok(users)
}
