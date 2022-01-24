use dotenv::dotenv;
use std::env;
use std::net::IpAddr;

use jdw::logger;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logger::init();

    let _env_host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let host: IpAddr = _env_host.parse().expect("failed to parse env HOST value");

    let _env_port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port: u16 = _env_port.parse().expect("failed to parse env PORT vaelue");

    warp::serve(jdw::server()).run((host, port)).await;
}
