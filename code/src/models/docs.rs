use serde::Serialize;

use super::graphql::{SignInResponse, SignUpResponse};

#[derive(Serialize)]
pub struct DocumentationType {
    pub how_to: &'static str,
    pub endpoint: &'static str,
    pub query: DocumentationQuery,
    pub mutation: DocumentationMutation,
}


#[derive(Serialize)]
pub struct DocumentationQuery {
    pub signin: DocumentationInput<SignInQueryInput, SignInResponse>
}

#[derive(Serialize)]
pub struct DocumentationMutation {
    pub signup: DocumentationInput<SignUpMutationInput, SignUpResponse>
}

#[derive(Serialize)]
pub struct DocumentationInput<K, V> {
    pub http_verb: &'static str,
    pub input: K,
    pub response: V,
    pub error: V
}

#[derive(Serialize)]
pub struct SignInQueryInput {
    pub email: String, 
    pub password: String
}

#[derive(Serialize)]
pub struct SignUpMutationInput {
    pub email: String, 
    pub password: String, 
    pub confirm_password: String
}