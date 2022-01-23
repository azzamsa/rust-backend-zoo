use juniper::FieldResult;

use crate::health;
use crate::health::schema::Health;
use crate::meta;
use crate::meta::schema::Meta;

use super::{Context, Query};

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub fn health(_ctx: &Context) -> FieldResult<Health> {
        health::service::read()
    }
    pub fn meta(_ctx: &Context) -> FieldResult<Meta> {
        meta::service::read()
    }
}
