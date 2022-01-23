pub mod schema;

use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> DbPool {
    let url = env::var("DATABASE_URL").expect("no `DATABASE_URL` specified ");
    let manager = ConnectionManager::new(url);
    Pool::builder()
        .build(manager)
        .expect("failed to create db pool")
}
