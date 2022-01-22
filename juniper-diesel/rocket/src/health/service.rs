use juniper::FieldResult;

use super::schema::Health;

pub fn read() -> FieldResult<Health> {
    let health = Health {
        status: "running".to_string(),
    };
    Ok(health)
}
