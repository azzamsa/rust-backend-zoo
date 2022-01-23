use cynic::QueryBuilder;
use rocket::http::{ContentType, Status};

use super::graphql::queries::MetaQuery;
use super::schema::Response;

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
