use cynic::QueryBuilder;
use rocket::http::{ContentType, Status};

use super::graphql::queries::HealthQuery;
use super::schema::Response;

#[test]
fn test_health() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).unwrap();
    let query = HealthQuery::build(());

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let response = resp.into_json::<Response>().unwrap();
    assert_eq!(response.data.health.status, "running");
}
