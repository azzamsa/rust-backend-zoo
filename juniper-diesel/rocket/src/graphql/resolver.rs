use juniper::FieldResult;

use crate::health;

#[juniper::graphql_object()]
impl Query {
    #[graphql(name = "Health")]
    pub fn get_health() -> FieldResult<health::schema::Health> {
        health::service::read()
    }
}
