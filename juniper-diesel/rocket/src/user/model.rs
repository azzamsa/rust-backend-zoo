use anyhow::{bail, Context};
use diesel::prelude::*;
use diesel::result::Error::NotFound;

use crate::diesel::RunQueryDsl;

use crate::db::DbPool;

use super::schema::{CreateUserInput, User};
use crate::db::schema::user_ as user;

pub fn find_all(pool: &DbPool) -> anyhow::Result<Vec<User>> {
    let users = user::table
        .load::<User>(&pool.get()?)
        .context("failed to perform a query to read users")?;

    Ok(users)
}
pub fn find(pool: &DbPool, id: i32) -> anyhow::Result<User> {
    let row = user::table
        .filter(user::id.eq(id))
        .first::<User>(&pool.get()?);

    match row {
        Ok(user) => Ok(user),
        Err(error) => {
            log::error!(
                "{}",
                format!("failed to perform a query to find user `{}`", error)
            );
            match error {
                NotFound => {
                    bail!("user not found")
                }
                _ => bail!("unexpected error when performing a query to find user"),
            }
        }
    }
}

pub fn create(pool: &DbPool, user_input: CreateUserInput) -> anyhow::Result<User> {
    let user = diesel::insert_into(user::table)
        .values(user_input)
        .get_result::<User>(&pool.get()?)
        .context("failed to perform a query to insert users")?;

    Ok(user)
}
