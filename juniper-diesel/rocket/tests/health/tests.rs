use anyhow::{Context, Result};
use cynic::QueryBuilder;
use rocket::http::{ContentType, Status};

use super::graphql::queries::HealthQuery;
use super::schema::HealthResponse;

#[test]
fn health() -> Result<()> {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).context("failed to create rocket test client")?;
    let query = HealthQuery::build(());

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let health_response = resp
        .into_json::<HealthResponse>()
        .context("failed to deserialize response")
        .map_err(|error| {
            log::trace!("response has no value `{:?}`", error);
            error
        })?;
    assert_eq!(health_response.data.health.status, "running");

    Ok(())
}
