#![allow(non_snake_case)]
use serde::{ Serialize, Deserialize };

#[derive(Serialize)]
pub struct FirebaseProfileRequest<'t> {
    pub idToken: &'t str
}

#[derive(Deserialize)]
pub struct FirebaseProfileResponse {
    pub users: Vec<FirebaseUserProfile>
}

#[derive(Deserialize, Clone)]
pub struct FirebaseUserProfile {
    pub localId: String,
    pub email: String,
    pub passwordHash: String,
    pub emailVerified: bool,
    pub passwordUpdatedAt: u128,
    pub providerUserInfo: Vec<FirebaseProviderUserInfo>,
    pub validSince: String,
    pub lastLoginAt: String,
    pub createdAt: String,
    pub lastRefreshAt: String,
    // pub photoUrl: String,
    // pub disabled: bool,
    // pub customAuth: bool
}

#[derive(Deserialize, Clone)]
pub struct FirebaseProviderUserInfo {
    pub providerId: String,
    pub federatedId: String,
    pub email: String,
    pub rawId: String
}
