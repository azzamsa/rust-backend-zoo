use anyhow::{Context, Result};
use cynic::{MutationBuilder, QueryBuilder};
use dotenv::dotenv;
use serde_json::{from_slice, Value};
use warp::http::StatusCode;
use warp::test::request;

use super::graphql::add;
use super::graphql::delete;
use super::graphql::queries::{ReadUserArguments, UserQuery, UsersQuery};
use super::graphql::update;

use super::schema::{CreateUserResponse, UpdateUserResponse};

#[tokio::test]
async fn read_users() -> Result<()> {
    dotenv().ok();
    let client = jdw::server();

    let query = UsersQuery::build(());
    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn read_user() -> Result<()> {
    dotenv().ok();
    let client = jdw::server();
    let args = ReadUserArguments { id: 100 };
    let query = UserQuery::build(args);

    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = from_slice(resp.body()).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    Ok(())
}

#[tokio::test]
async fn create_user() -> Result<()> {
    dotenv().ok();
    let client = jdw::server();

    let args = add::CreateUserInput {
        name: "khawa-create".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let user_response: CreateUserResponse =
        from_slice(resp.body()).context("failed to deserialize response body")?;

    assert_eq!(user_response.data.create_user.name, "khawa-create");

    Ok(())
}

#[tokio::test]
async fn duplicate_username() -> Result<()> {
    dotenv().ok();
    let client = jdw::server();

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-duplicate".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    //
    // Create next user with the same name
    //

    let args = add::CreateUserInput {
        name: "khawa-duplicate".to_string(),
        full_name: None,
    };
    let query = add::UserMutation::build(&args);

    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    let body: Value = from_slice(resp.body()).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "a user with same `name` already exists");

    Ok(())
}

#[tokio::test]
async fn update_user() -> Result<()> {
    dotenv().ok();
    let client = jdw::server();

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-update".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let user_response: CreateUserResponse =
        from_slice(resp.body()).context("failed to deserialize response body")?;

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

    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    let user_response: UpdateUserResponse =
        from_slice(resp.body()).context("failed to deserialize response body")?;

    assert_eq!(user_response.data.update_user.name, "haitham");

    Ok(())
}

#[tokio::test]
async fn delete_user() -> Result<()> {
    dotenv().ok();
    let client = jdw::server();

    //
    // Create User
    //

    let args = add::CreateUserInput {
        name: "khawa-delete".to_string(),
        full_name: Some("Abu Musa Al-Khawarizmi".to_string()),
    };
    let query = add::UserMutation::build(&args);

    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let user_response: CreateUserResponse =
        from_slice(resp.body()).context("failed to deserialize response body")?;

    assert_eq!(user_response.data.create_user.name, "khawa-delete");
    let user_id = user_response.data.create_user.id;

    //
    // Update User
    //

    let args = delete::DeleteUserArguments { id: user_id };
    let query = delete::UserMutation::build(&args);

    let _ = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    //
    // Make sure user deleted
    //

    let args = ReadUserArguments { id: user_id };
    let query = UserQuery::build(args);

    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = from_slice(resp.body()).context("failed to deserialize response")?;
    let error_message = &body["errors"][0]["message"];
    assert_eq!(error_message, "user not found");

    Ok(())
}
