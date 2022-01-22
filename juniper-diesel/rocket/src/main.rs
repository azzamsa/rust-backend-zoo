use zoo::logger;

#[rocket::launch]
fn rocket() -> _ {
    logger::init();

    zoo::rocket()
}
