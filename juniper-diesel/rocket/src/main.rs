use jdr::logger;

#[rocket::launch]
fn rocket() -> _ {
    logger::init();

    jdr::rocket()
}
