mod resolver;

use crate::db::DbPool;

use juniper::{EmptySubscription, RootNode};

pub struct Context {
    pub pool: DbPool,
}
pub struct Mutation;
pub struct Query;

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;
