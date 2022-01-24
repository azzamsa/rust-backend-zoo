use adr::logger;

#[rocket::launch]
fn rocket() -> _ {
    logger::init();

    adr::rocket()
}
