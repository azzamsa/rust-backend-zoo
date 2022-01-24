use anyhow::{Context, Result};
use cynic::QueryBuilder;
use dotenv::dotenv;
use serde_json::from_slice;
use warp::http::StatusCode;
use warp::test::request;

use super::graphql::queries::HealthQuery;
use super::schema::HealthResponse;

#[tokio::test]
async fn health() -> Result<()> {
    dotenv().ok();
    let client = jdw::server();

    let query = HealthQuery::build(());
    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let health_response: HealthResponse =
        from_slice(resp.body()).context("failed to serialize response body")?;
    assert_eq!(health_response.data.health.status, "running");

    Ok(())
}
