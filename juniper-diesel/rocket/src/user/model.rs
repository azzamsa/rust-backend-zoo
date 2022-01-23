use anyhow::Context;

use crate::diesel::RunQueryDsl;

use crate::db::DbPool;

use super::schema::User;
use crate::db::schema::user_ as user;

pub fn find_all(pool: &DbPool) -> anyhow::Result<Vec<User>> {
    let users = user::table
        .load::<User>(&pool.get()?)
        .context("failed to perform a query to read users")?;

    Ok(users)
}
