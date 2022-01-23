mod resolver;

use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub struct Context;
pub struct Query;

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
