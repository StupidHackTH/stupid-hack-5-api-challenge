#![allow(non_snake_case)]
use juniper::GraphQLObject;

use serde::Serialize;

#[derive(Serialize, GraphQLObject, Clone)]
pub struct SignUpResponse {
    pub success: bool,
    pub info: String
}

#[derive(Serialize, GraphQLObject, Clone)]
pub struct SignInResponse {
    pub success: bool,
    pub info: String,
    pub kind: Option<String>,
    pub idToken: Option<String>,
    pub refreshToken: Option<String>,
    pub expiresIn: Option<i32>
}
