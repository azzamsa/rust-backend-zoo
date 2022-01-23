use serde::Deserialize;

use super::graphql::queries::Meta;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub data: MetaWrapper,
}

#[derive(Debug, Deserialize)]
pub struct MetaWrapper {
    pub meta: Meta,
}
