mod resolver;

use crate::db::DbPool;

use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub struct Context {
    pub pool: DbPool,
}
pub struct Query;

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
