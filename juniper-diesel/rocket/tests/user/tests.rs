use cynic::{MutationBuilder, QueryBuilder};
use rocket::http::{ContentType, Status};
use serde_json::{from_slice, Value};

use super::graphql::mutations::{CreateUserInput, UserMutation};
use super::graphql::queries::{ReadUserArguments, UserQuery, UsersQuery};

use super::schema::CreateUserResponse;

#[test]
fn read_users() {
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
fn read_user() {
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

#[test]
fn create_user() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).unwrap();

    let args = CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = UserMutation::build(&args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let response = resp.into_json::<CreateUserResponse>().unwrap();
    assert_eq!(response.data.create_user.name, "khawa");
}

#[test]
fn duplicate_username() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).unwrap();

    let args = CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = UserMutation::build(&args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let response = resp.into_json::<CreateUserResponse>().unwrap();
    assert_eq!(response.data.create_user.name, "khawa");

    // Create next user with the same name
    let args = CreateUserInput {
        name: "khawa".to_string(),
        full_name: None,
    };
    let query = UserMutation::build(&args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    let body: Value = from_slice(&resp.into_bytes().unwrap()).unwrap();
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "a user with same `name` already exists");
}
