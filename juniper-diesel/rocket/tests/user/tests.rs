use anyhow::{Context, Result};
use cynic::{MutationBuilder, QueryBuilder};
use rocket::http::{ContentType, Status};
use serde_json::{from_slice, Value};

use super::graphql::mutations::{CreateUserInput, UserMutation};
use super::graphql::queries::{ReadUserArguments, UserQuery, UsersQuery};

use super::schema::CreateUserResponse;
use crate::utils::cleanup_db;

#[test]
fn read_users() -> Result<()> {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).context("failed to create rocket test client")?;
    let query = UsersQuery::build(());

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    Ok(())
}

#[test]
fn read_user() -> Result<()> {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).context("failed to create rocket test client")?;
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

    Ok(())
}

#[test]
fn create_user() -> Result<()> {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).context("failed to create rocket test client")?;

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

    let user_response = resp
        .into_json::<CreateUserResponse>()
        .context("failed to deserialize response")
        .map_err(|error| {
            log::trace!("response has no value `{:?}`", error);
            error
        })?;

    assert_eq!(user_response.data.create_user.name, "khawa");

    // Tear down
    cleanup_db();

    Ok(())
}

#[test]
fn duplicate_username() -> Result<()> {
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

    let resp_bytes = resp
        .into_bytes()
        .context("failed to deserialize response")
        .map_err(|error| {
            log::trace!("response has no value `{:?}`", error);
            error
        })?;

    let body: Value = from_slice(&resp_bytes).context("failed to serialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "a user with same `name` already exists");

    // Tear down
    cleanup_db();

    Ok(())
}
