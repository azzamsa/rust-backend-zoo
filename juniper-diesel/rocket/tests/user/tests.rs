use anyhow::{Context, Result};
use cynic::{MutationBuilder, QueryBuilder};
use rocket::http::{ContentType, Status};
use serde_json::{from_slice, Value};

use super::graphql::add;
use super::graphql::delete;
use super::graphql::queries::{ReadUserArguments, UserQuery, UsersQuery};
use super::graphql::update;

use super::schema::{CreateUserResponse, UpdateUserResponse};
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

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

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

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    // Create next user with the same name
    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: None,
    };
    let query = add::UserMutation::build(&args);

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

#[test]
fn update_user() -> Result<()> {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).context("failed to create rocket test client")?;

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

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
    let user_id = user_response.data.create_user.id;

    // Update User

    let args = update::UpdateUserInput {
        id: user_id,
        name: "haitham".to_string(),
        full_name: None,
    };
    let query = update::UserMutation::build(&args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    let user_response = resp
        .into_json::<UpdateUserResponse>()
        .context("failed to deserialize response")
        .map_err(|error| {
            log::trace!("response has no value `{:?}`", error);
            error
        })?;

    assert_eq!(user_response.data.update_user.name, "haitham");

    // Tear down
    cleanup_db();

    Ok(())
}

#[test]
fn delete_user() -> Result<()> {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).context("failed to create rocket test client")?;

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

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
    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //

    let args = delete::DeleteUserArguments { id: user_id };
    let query = delete::UserMutation::build(&args);

    let _ = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    //
    // Make sure user deleted
    //

    let args = ReadUserArguments { id: user_id };
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

    // Tear down
    cleanup_db();

    Ok(())
}
