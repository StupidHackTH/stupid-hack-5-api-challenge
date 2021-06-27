use std::env;

use juniper::{graphql_object, EmptySubscription, RootNode};

use crate::models::{firebase::{FirebaseEmailSignError, FirebaseEmailSignUp, FirebaseToken}, graphql::{SignInResponse, SignUpResponse}};

use reqwest::Client;

pub struct Query;

#[graphql_object]
impl Query {
    pub async fn signin(email: String, password: String) -> SignInResponse {
        let http = Client::new();
        let key = env::var("firebase_api_key").expect("Firebase API Key not found");
        let url = format!("https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}", key);

        let request = http.post(url)
            .json(&FirebaseEmailSignUp {
                email,
                password,
                returnSecureToken: true
            })
            .send()
            .await;

        if let Err(error) = request {
            return SignInResponse {
                success: false,
                info: error.to_string(),
                kind: None,
                id_token: None,
                refresh_token: None,
                expires_in: None
            }
        }

        let response = request.unwrap();
        let response_string = response.text().await.unwrap().clone();

        let error = serde_json::from_str::<FirebaseEmailSignError>(&response_string);

        if let Ok(error_response) = error {
            return SignInResponse {
                success: false,
                info: error_response.error.errors[0].clone().message,
                kind: None,
                id_token: None,
                refresh_token: None,
                expires_in: None
            }
        }

        let token_response = serde_json::from_str::<FirebaseToken>(&response_string);

        if let Ok(token) = token_response {
            return SignInResponse {
                success: true,
                info: "Sign in successfully".to_owned(),
                kind: Some(token.kind),
                id_token: Some(token.idToken),
                refresh_token: Some(token.refreshToken),
                expires_in: Some(token.expiresIn.parse::<i32>().unwrap())
            }
        }

        SignInResponse {
            success: false,
            info: "Something went wrong".to_owned(),
            kind: None,
            id_token: None,
            refresh_token: None,
            expires_in: None
        }
    }
}

pub struct Mutation;

#[graphql_object]
impl Mutation {
    pub async fn signup(email: String, password: String, confirm_password: String) -> SignUpResponse {
        if password != confirm_password {
            return SignUpResponse {
                success: false,
                info: "Confirm password is not match".to_owned()
            }
        }

        let http = Client::new();
        let key = env::var("firebase_api_key").expect("Firebase API Key not found");
        let url = format!("https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}", key);

        let request = http.post(url)
            .json(&FirebaseEmailSignUp {
                email,
                password,
                returnSecureToken: true
            })
            .send()
            .await;

        if let Err(error) = request {
            return SignUpResponse {
                success: false,
                info: error.to_string()
            }
        }

        let response = request.unwrap();

        let error = response.json::<FirebaseEmailSignError>().await;

        if let Ok(error_response) = error {
            return SignUpResponse {
                success: false,
                info: error_response.error.errors[0].clone().message
            }
        }

        SignUpResponse {
            success: true,
            info: "Sign up successfully".to_owned()
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query {},
        Mutation {},
        EmptySubscription::new()
    )
}