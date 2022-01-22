use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub type AppSchema = RootNode<'static, Query, EmptyMutation, EmptySubscription>;

pub struct Context;
pub struct Query;

pub fn build() -> AppSchema {
    AppSchema::new(Query, EmptyMutation)
}
