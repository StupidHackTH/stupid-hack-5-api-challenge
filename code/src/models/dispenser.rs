use serde::{ Serialize, Deserialize };
use juniper::GraphQLObject;

#[derive(Serialize)]
pub struct DispenserRequest {
    pub key: String,
    pub email: String,
    pub production: bool
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct DispenserResponse {
    pub success: bool,
    pub info: String,
    pub ticket: Option<String>
}