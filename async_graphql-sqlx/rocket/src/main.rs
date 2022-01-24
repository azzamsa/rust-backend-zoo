use asr::logger;

#[rocket::launch]
fn rocket() -> _ {
    logger::init();

    asr::rocket()
}
