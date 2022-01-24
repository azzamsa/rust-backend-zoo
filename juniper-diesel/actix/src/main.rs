use actix_web::{web, App, HttpServer};
use anyhow::Context;
use dotenv::dotenv;
use juniper::EmptySubscription;

use jda::graphql::{Mutation, Query, Schema};
use jda::logger;
use jda::routes::{graphiql_route, graphql_playground, graphql_route};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load .env
    dotenv().ok();

    logger::init();

    let schema = Schema::new(
        Query,
        Mutation,
        EmptySubscription::<graphql::Context>::new(),
    );

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(graphql_playground())))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
    });

    let address = format!(
        "{}:{}",
        env::var("HOST").expect("HOST env is not set"),
        env::var("PORT").expect("PORT env is not set")
    );
    server.bind(address)?.run().await
}
