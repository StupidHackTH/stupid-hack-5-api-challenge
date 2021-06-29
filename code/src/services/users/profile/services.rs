use std::env;

use reqwest::Client;

use crate::models::firebase::{FirebaseEmailErrorInfo, FirebaseEmailSignError, FirebaseEmailSignErrorDetail};

use super::models::{ FirebaseProfileRequest, FirebaseUserProfile, FirebaseProfileResponse };

pub async fn get_profile<'t>(id: &str) -> Result<FirebaseUserProfile, FirebaseEmailSignError> {
    let http = Client::new();
    let key = env::var("firebase_api_key").expect("Firebase API Key not found");
    let url = format!("https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}", key);

    let request = http.post(url)
        .json(&FirebaseProfileRequest {
            idToken: id
        })
        .send()
        .await
        .unwrap();

    let response = request.text().await.unwrap();

    if let Ok(error) = serde_json::from_str::<FirebaseEmailSignError>(&response) {
        return Err(error);
    }

    if let Ok(response) = serde_json::from_str::<FirebaseProfileResponse>(&response) {
        Ok(response.users[0].clone())
    } else {
        if let Err(error) = serde_json::from_str::<FirebaseProfileResponse>(&response) {
            println!("{:?}", response);
            println!("{:?}", error);
        }

        Err(FirebaseEmailSignError {
            error: FirebaseEmailErrorInfo {
                code: 502,
                message: "Something went wrong".to_owned(),
                errors: vec![
                    FirebaseEmailSignErrorDetail {
                        message: "Something went wrong".to_owned(),
                        domain: "global".to_owned(),
                        reason: "invalid".to_owned()
                    }
                ]
            }
        })
    }
}
