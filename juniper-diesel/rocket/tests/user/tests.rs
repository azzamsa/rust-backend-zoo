use cynic::QueryBuilder;
use rocket::http::{ContentType, Status};

use super::graphql::queries::UsersQuery;

#[test]
fn test_user() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).unwrap();
    let query = UsersQuery::build(());

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);
}
