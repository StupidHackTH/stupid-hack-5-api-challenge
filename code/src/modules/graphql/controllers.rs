use std::sync::Arc;
use actix_web::{ get, post, web::{ ServiceConfig, Json, Data }, HttpResponse, Error };

use juniper::http::{ GraphQLRequest, graphiql::graphiql_source };

use crate::services::schema::Schema;

#[get("/status")]
async fn status() -> &'static str {
    "Working"
}

#[post("/graphql")]
async fn graphql(data: Data<Arc<Schema>>, request: Json<GraphQLRequest>) -> Result<HttpResponse, Error> {
    let response = request.execute(&data, &()).await;

    Ok(
        HttpResponse::Ok()
            .json(response)
    )
}

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .body(
            graphiql_source("/graphql", None)
        )
}

pub fn api(config: &mut ServiceConfig) {
    config
        .service(status)
        // .service(graphiql)
        .service(graphql);
}
