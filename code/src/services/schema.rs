use std::env;

use juniper::{graphql_object, EmptySubscription, RootNode, Context};

use crate::models::{
    firebase::{
        FirebaseEmailSignError, 
        FirebaseEmailSignUp, 
        FirebaseToken
    }, 
    graphql::{
        ReserveTicketResponse, 
        SignInResponse, 
        SignUpResponse
    }
};

use reqwest::{ Client, header };

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

        let error = request.unwrap().json::<FirebaseEmailSignError>().await;

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

    pub async fn reserve(context: &RequestContext, note: Option<String>) -> ReserveTicketResponse {
        let profile = get_profile(&context.authorization.replace("Bearer ", "")).await;

        if profile.is_err() {
            let error = profile.err().unwrap().error;

            return ReserveTicketResponse {
                success: false,
                info: error.message,
                ticketId: None
            };
        }

        let profile = profile.unwrap();
        let project_id = env::var("project_id").expect("Firebase Project Id not found");
        let http = Client::new();

        let url = format!("https://firestore.googleapis.com/v1/projects/{}/databases/(default)/collectionGroups/reserve/{}", project_id, profile.localId);
   
        let request = http
            .post(url)
            .header(header::AUTHORIZATION, &context.authorization)
            .send()
            .await
            .unwrap();

        let response = request.text().await.unwrap();

        println!("{:?}", response);

        ReserveTicketResponse {
            success: true,
            info: "Reserved successfully".to_owned(),
            ticketId: None
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