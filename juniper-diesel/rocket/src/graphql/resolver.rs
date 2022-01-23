use juniper::FieldResult;

use crate::health;
use crate::health::schema::Health;

use super::{Context, Query};

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub fn health(_ctx: &Context) -> FieldResult<Health> {
        health::service::read()
    }
}
