use anyhow::{Context, Result};
use cynic::QueryBuilder;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

use super::graphql::queries::MetaQuery;
use super::schema::MetaResponse;

#[test]
fn meta() -> Result<()> {
    let client = Client::tracked(adr::rocket()).context("failed to create rocket test client")?;
    let query = MetaQuery::build(());

    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .json(&query)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let meta_response = resp
        .into_json::<MetaResponse>()
        .context("failed to deserialize response")
        .map_err(|error| {
            log::trace!("response has no value `{:?}`", error);
            error
        })?;
    let cargo_package_version = env!("CARGO_PKG_VERSION").to_string();
    assert_eq!(meta_response.data.meta.version, cargo_package_version);

    Ok(())
}
