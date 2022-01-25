// After some experiment, we avoid using the setup and tear-down method
// of cleaning up the database after and before each test.
//
// Currently, Rust has no good support for something like Python `Conftest.py`.
// We have to do some acrobats to do setup-and-teardown.
//
// Most of the time, the tests are failing, because the teardown is not executed if
// the test function is panic.
//
// The best workaround we have currently is just to use different fixture for each test
// function.

use anyhow::{Context, Result};
use cynic::{MutationBuilder, QueryBuilder};
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json::{from_slice, Value};

use super::graphql::add;
use super::graphql::delete;
use super::graphql::queries::{ReadUserArguments, UserQuery, UsersQuery};
use super::graphql::update;

use super::schema::{CreateUserResponse, UpdateUserResponse};

#[test]
fn read_users() -> Result<()> {
    let client = Client::tracked(asr::rocket()).context("failed to create rocket test client")?;
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
    let client = Client::tracked(asr::rocket()).context("failed to create rocket test client")?;
    let args = ReadUserArguments { id: 100 };
    let query = UserQuery::build(args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let resp_bytes = resp
        .into_bytes()
        .context("failed to deserialize response")?;

    let body: Value = from_slice(&resp_bytes).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    Ok(())
}

#[test]
fn create_user() -> Result<()> {
    let client = Client::tracked(asr::rocket()).context("failed to create rocket test client")?;

    let args = add::CreateUserInput {
        name: "khawa-create".to_string(),
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
        .context("failed to deserialize response")?;

    assert_eq!(user_response.data.create_user.name, "khawa-create");

    Ok(())
}

#[test]
fn create_user_without_full_name() -> Result<()> {
    let client = Client::tracked(asr::rocket()).context("failed to create rocket test client")?;

    let args = add::CreateUserInput {
        name: "khawa-create-no-full-name".to_string(),
        full_name: None,
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
        .context("failed to deserialize response")?;

    assert_eq!(
        user_response.data.create_user.name,
        "khawa-create-no-full-name"
    );
    assert_eq!(user_response.data.create_user.full_name, None);

    Ok(())
}

#[test]
fn duplicate_username() -> Result<()> {
    let client = Client::tracked(asr::rocket()).context("failed to create rocket test client")?;

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-duplicate".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    //
    // Create next user with the same name
    //

    let args = add::CreateUserInput {
        name: "khawa-duplicate".to_string(),
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
        .context("failed to deserialize response")?;

    let body: Value = from_slice(&resp_bytes).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "a user with same `name` already exists");

    Ok(())
}

#[test]
fn update_user() -> Result<()> {
    let client = Client::tracked(asr::rocket()).context("failed to create rocket test client")?;

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-update".to_string(),
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
        .context("failed to deserialize response")?;

    assert_eq!(user_response.data.create_user.name, "khawa-update");
    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //

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
        .context("failed to deserialize response")?;

    assert_eq!(user_response.data.update_user.name, "haitham");

    Ok(())
}

#[test]
fn delete_user() -> Result<()> {
    let client = Client::tracked(asr::rocket()).context("failed to create rocket test client")?;

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-delete".to_string(),
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
        .context("failed to deserialize response")?;

    assert_eq!(user_response.data.create_user.name, "khawa-delete");
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

    let resp_bytes = resp
        .into_bytes()
        .context("failed to deserialize response")?;

    let body: Value = from_slice(&resp_bytes).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    Ok(())
}

#[test]
fn keep_existing_full_name() -> Result<()> {
    let client = Client::tracked(asr::rocket()).context("failed to create rocket test client")?;

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-keep".to_string(),
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
        .context("failed to deserialize response")?;

    assert_eq!(user_response.data.create_user.name, "khawa-keep");
    let user_id = user_response.data.create_user.id;

    //
    // Update Only the user name
    //

    let args = update::UpdateUserInput {
        id: user_id,
        name: "khawa-keep-2".to_string(),
        full_name: None,
    };
    let query = update::UserMutation::build(&args);

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    //
    // Make sure the full name preserved
    //

    let user_response = resp
        .into_json::<UpdateUserResponse>()
        .context("failed to deserialize response")?;

    assert_eq!(user_response.data.update_user.name, "khawa-keep-2");
    assert_eq!(
        user_response.data.update_user.full_name,
        Some("Abu Musa Al-Khawarizmi".to_string())
    );

    Ok(())
}
