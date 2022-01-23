use serde::Deserialize;

use super::graphql::queries::Health;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub data: HealthWrapper,
}

#[derive(Debug, Deserialize)]
pub struct HealthWrapper {
    pub health: Health,
}
