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
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i32>
}
