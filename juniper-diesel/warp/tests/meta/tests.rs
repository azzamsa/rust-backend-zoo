use anyhow::{Context, Result};
use cynic::QueryBuilder;
use dotenv::dotenv;
use serde_json::from_slice;
use warp::http::StatusCode;
use warp::test::request;

use super::graphql::queries::MetaQuery;
use super::schema::MetaResponse;

#[tokio::test]
async fn meta() -> Result<()> {
    dotenv().ok();
    let client = jdw::server();

    let query = MetaQuery::build(());
    let resp = request()
        .method("POST")
        .path("/graphql")
        .json(&query)
        .reply(&client)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let meta_response: MetaResponse =
        from_slice(resp.body()).context("failed to deserialize response body")?;
    let cargo_package_version = env!("CARGO_PKG_VERSION").to_string();
    assert_eq!(meta_response.data.meta.version, cargo_package_version);

    Ok(())
}
