#![allow(non_snake_case)]
use std::env;

use juniper::{graphql_object, EmptySubscription, RootNode, Context};

use crate::models::{
    dispenser::{
        DispenserRequest, 
        DispenserResponse
    }, 
    firebase::{
        FirebaseEmailSignError, 
        FirebaseEmailSignUp, 
        FirebaseToken
    }, 
    graphql::{
        SignInResponse, 
        SignUpResponse
    }
};

use reqwest::Client;

use super::users::profile::services::get_profile;

pub struct RequestContext {
    pub authorization: String
}

impl Context for RequestContext {}

pub struct Query;

#[graphql_object(context = RequestContext)]
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
                localId: None,
                idToken: None,
                refreshToken: None,
                expiresIn: None
            }
        }

        let response_string = request.unwrap().text().await.unwrap().clone();

        let error = serde_json::from_str::<FirebaseEmailSignError>(&response_string);

        if let Ok(error_response) = error {
            return SignInResponse {
                success: false,
                info: error_response.error.errors[0].clone().message,
                kind: None,
                localId: None,
                idToken: None,
                refreshToken: None,
                expiresIn: None
            }
        }

        let token_response = serde_json::from_str::<FirebaseToken>(&response_string);

        if let Ok(token) = token_response {
            return SignInResponse {
                success: true,
                info: "Sign in successfully".to_owned(),
                kind: Some(token.kind),
                localId: Some(token.localId),
                idToken: Some(token.idToken),
                refreshToken: Some(token.refreshToken),
                expiresIn: Some(token.expiresIn.parse::<i32>().unwrap())
            }
        }

        SignInResponse {
            success: false,
            info: "Something went wrong".to_owned(),
            kind: None,
            localId: None,
            idToken: None,
            refreshToken: None,
            expiresIn: None
        }
    }
}

pub struct Mutation;

#[graphql_object(context = RequestContext)]
impl Mutation {
    pub async fn signup(email: String, password: String, confirmPassword: String) -> SignUpResponse {
        if password != confirmPassword {
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

        let error = request.unwrap().json::<FirebaseEmailSignError>().await;

        if let Ok(response) = error {
            return SignUpResponse {
                success: false,
                info: response
                    .error
                    .errors[0]
                    .clone()
                    .message
            }
        }

        SignUpResponse {
            success: true,
            info: "Sign up successfully".to_owned()
        }
    }

    pub async fn reserve(context: &RequestContext) -> DispenserResponse {
        let profile = get_profile(&context.authorization.replace("Bearer ", "")).await;

        if profile.is_err() {
            let error = profile.err().unwrap().error;

            return DispenserResponse {
                success: false,
                info: error.message,
                ticket: None
            };
        }

        let profile = profile.unwrap();
        let http = Client::new();
        let challenge_key = env::var("challenge_key").expect("API Challenge key not found");
        let url = "https://sth5-dispenser.saltyaom.com/dispenser/unassign-challenge";
        
        let request = http.post(url)
            .json(&DispenserRequest {
                email: profile.email,
                key: challenge_key,
                production: env::var("production").unwrap_or("".to_owned()) == "true"
            })
            .send()
            .await;

            if let Err(error) = request {
                return DispenserResponse {
                    success: false,
                    info: error.to_string(),
                    ticket: None
                }
            }
    
        let dispenser = request.unwrap().json::<DispenserResponse>().await;

        match dispenser {
            Ok(dispense) => {
                if dispense.success {
                    DispenserResponse {
                        success: true,
                        info: "Reserved successfully, please use the ticket code to serve ticket at EventPop.".to_owned(),
                        ticket: dispense.ticket
                    }
                } else {
                    DispenserResponse {
                        success: false,
                        info: dispense.info,
                        ticket: None
                    }
                }
            },
            Err(error) => {
                DispenserResponse {
                    success: false,
                    info: error.to_string(),
                    ticket: None
                }
            }
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<RequestContext>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query {},
        Mutation {},
        EmptySubscription::new()
    )
}