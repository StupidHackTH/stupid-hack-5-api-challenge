mod modules;
mod models;
mod services;

use std::{ env, sync::Arc };
use dotenv::dotenv;

use actix_web::{ 
    HttpServer, 
    App, 
    middleware::Compress, 
    web::{ 
        route, 
        Data 
    } 
};

use modules::{
    landing::controllers::landing,
    docs::controllers::docs,
    graphql::controllers::api, 
    not_found::controllers::not_found
};

use services::schema::create_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::var("firebase_api_key").expect("Firebae API Key");
    env::var("admin_id").expect("Admin key");
    env::var("project_id").expect("Firebae Project Id");
    env::var("challenge_key").expect("API Challenge key");

    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(Data::new(schema.clone()))
            .service(landing)
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