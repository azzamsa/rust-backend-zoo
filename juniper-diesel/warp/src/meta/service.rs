use juniper::FieldResult;

use super::schema::Meta;

pub fn read() -> FieldResult<Meta> {
    let meta = Meta {
        build: option_env!("VCS_REVISION").unwrap_or("unknown").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(meta)
}
