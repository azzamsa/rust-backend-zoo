#[rocket::get("/")]
pub const fn hello() -> &'static str {
    "Hello, world!"
}
