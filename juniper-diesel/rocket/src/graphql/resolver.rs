use juniper::FieldResult;

use crate::health;
use crate::health::schema::Health;
use crate::meta;
use crate::meta::schema::Meta;
use crate::user;
use crate::user::schema::User;

use super::{Context, Query};

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub fn health(_ctx: &Context) -> FieldResult<Health> {
        health::service::read()
    }
    pub fn meta(_ctx: &Context) -> FieldResult<Meta> {
        meta::service::read()
    }
    pub fn users(ctx: &Context) -> FieldResult<Vec<User>> {
        let pool = &ctx.pool;
        user::service::read_all(pool)
    }
    pub fn user(ctx: &Context, id: i32) -> FieldResult<User> {
        let pool = &ctx.pool;
        user::service::read(pool, id)
    }
}
