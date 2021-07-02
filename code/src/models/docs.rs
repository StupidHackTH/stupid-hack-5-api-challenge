#![allow(non_snake_case)]
use serde::Serialize;

use super::{
    dispenser::DispenserResponse, 
    graphql::{
        SignInResponse,
        SignUpResponse
    }
};

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
    pub signup: DocumentationInput<SignUpMutationInput, SignUpResponse>,
    pub reserve: DocumentationInput<(), DispenserResponse>
}

#[derive(Serialize)]
pub struct DocumentationInput<K, V> {
    pub httpVerb: &'static str,
    pub header: DocumentationHeaderRequest,
    pub input: Option<K>,
    pub response: V,
    pub error: V
}

#[derive(Serialize)]
pub struct DocumentationHeaderRequest {
    pub authorization: Option<&'static str>
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
    pub confirmPassword: String
}