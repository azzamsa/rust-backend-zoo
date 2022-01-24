use diesel::prelude::*;
use zoo::db::get_pool;
use zoo::db::schema::user_::dsl::user_;

/// A teardown function to cleanup database after each test
pub fn cleanup_db() {
    let conn = get_pool()
        .expect("failed to get db pool")
        .get()
        .expect("failed to connect to db");
    diesel::delete(user_)
        .execute(&conn)
        .expect("failed to delete user table");
}
