use cynic::QueryBuilder;
use rocket::http::{ContentType, Status};
use serde_json::{from_slice, Value};

use super::graphql::queries::{ReadUserArguments, UserQuery, UsersQuery};

#[test]
fn test_read_users() {
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

#[test]
fn test_read_user() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).unwrap();
    let args = ReadUserArguments { id: 100 };
    let query = UserQuery::build(args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let body: Value = from_slice(&resp.into_bytes().unwrap()).unwrap();
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");
}
