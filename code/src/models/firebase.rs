#![allow(non_snake_case)]
use serde::{Serialize, Deserialize };

#[derive(Serialize)]
pub struct FirebaseEmailSignUp {
    pub email: String,
    pub password: String,
    pub returnSecureToken: bool
}

#[derive(Deserialize, Clone, Debug)]
pub struct FirebaseEmailSignError {
    pub error: FirebaseEmailSignUpErrorInfo
}

#[derive(Deserialize, Clone, Debug)]
pub struct FirebaseEmailSignUpErrorInfo {
    pub code: u16,
    pub message: String,
    pub errors: Vec<FirebaseEmailSignErrorDetail>
}


#[derive(Deserialize, Clone, Debug)]
pub struct FirebaseEmailSignErrorDetail {
    pub message: String,
    pub domain: String,
    pub reason: String
}

#[derive(Deserialize, Debug)]
pub struct FirebaseToken {
    pub kind: String,
    pub localId: String,
    pub email: String,
    pub displayName: String,
    pub idToken: String,
    pub registered: bool,
    pub refreshToken: String,
    pub expiresIn: String
}