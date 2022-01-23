use cynic::QueryBuilder;
use rocket::http::{ContentType, Status};
use serde::Deserialize;
use zoo::meta::schema::Meta;

use super::graphql::queries::MetaQuery;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub data: MetaWrapper,
}

#[derive(Debug, Deserialize)]
pub struct MetaWrapper {
    pub meta: Meta,
}

#[test]
fn test_meta() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).unwrap();
    let query = MetaQuery::build(());

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let response = resp.into_json::<Response>().unwrap();
    let cargo_package_version = env!("CARGO_PKG_VERSION").to_string();
    assert_eq!(response.data.meta.version, cargo_package_version);
}
