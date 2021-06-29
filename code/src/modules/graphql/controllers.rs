use std::sync::Arc;
use actix_web::{Error, HttpRequest, HttpResponse, get, post, web::{Data, Json, ServiceConfig}};

use juniper::http::{ GraphQLRequest, graphiql::graphiql_source };

use crate::services::schema::{RequestContext, Schema};

#[get("/status")]
async fn status() -> &'static str {
    "Working"
}

#[post("/graphql")]
async fn graphql(data: Data<Arc<Schema>>, graphql: Json<GraphQLRequest>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let authorization = request
        .headers()
        .get("Authorization");

    let authorization = if let Some(authorization) = authorization {
        authorization
            .to_str()
            .unwrap_or("")
            .to_owned()
    } else {
        "".to_owned()
    };

    let context = RequestContext {
        authorization
    };

    let response = graphql.execute(
        &data,
        &context
    ).await;

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
        .service(graphiql)
        .service(graphql);
}
