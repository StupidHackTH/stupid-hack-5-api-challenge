use actix_web::{HttpResponse, get, http::header, web::ServiceConfig};

use crate::models::{
    docs::{
        DocumentationInput, 
        DocumentationMutation, 
        DocumentationQuery, 
        DocumentationType, 
        SignInQueryInput, 
        SignUpMutationInput
    }, 
    graphql::{
        SignInResponse, 
        SignUpResponse
    }
};

#[get("/")]
async fn documentation() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(header::ContentType::html())
        .json(DocumentationType {
            how_to: "Create GraphQL request from the following data",
            endpoint: "/graphql",
            query: DocumentationQuery {
                signin: DocumentationInput {
                    http_verb: "POST",
                    input: SignInQueryInput {
                        email: "Your email".to_owned(),
                        password: "Your registered password".to_owned()
                    },
                    response: SignInResponse {
                        success: true,
                        info: "Response information".to_owned(),
                        kind: Some("Sign in type".to_owned()),
                        localId: Some("Your UID".to_owned()),
                        idToken: Some("Token to regenerate refresh token".to_owned()),
                        refreshToken: Some("Short-live refresh token".to_owned()),
                        expiresIn: Some(3600)
                    },
                    error: SignInResponse {
                        success: false,
                        info: "Error information".to_owned(),
                        kind: None,
                        localId: None,
                        idToken: None,
                        refreshToken: None,
                        expiresIn: None
                    }
                }
            },
            mutation: DocumentationMutation {
                signup: DocumentationInput {
                    http_verb: "POST",
                    input: SignUpMutationInput {
                        email: "Your email".to_owned(),
                        password: "Your registered password".to_owned(),
                        confirm_password: "Must match password field".to_owned()
                    },
                    response: SignUpResponse {
                        success: true,
                        info: "Response information".to_owned(),
                    },
                    error: SignUpResponse {
                        success: false,
                        info: "Error information".to_owned(),
                    }
                }
            }
        })
}

pub fn docs(config: &mut ServiceConfig) {
    config.service(documentation);
}