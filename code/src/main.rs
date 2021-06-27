mod modules;
mod models;
mod services;

use std::sync::Arc;
use dotenv::dotenv;

use actix_web::{ HttpServer, App, middleware::Compress, web::{ route, Data } };

use modules::{docs::controllers::docs, graphql::controllers::api, not_found::controllers::not_found};

use services::schema::create_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(Data::new(schema.clone()))
            .configure(api)
            .configure(docs)
            .default_service(
                route().to(not_found)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}