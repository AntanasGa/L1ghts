use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StepGetRes {
    pub step: String,
}

#[derive(Deserialize)]
pub struct SetupAuth {
    pub user: AuthReq,
    pub key: String,
}

#[derive(Deserialize, Clone)]
pub struct AuthReq {
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
    pub uid: i32,
    pub exp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshToken {
    pub uid: i32,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Refresh {
    pub token: String,
}

#[derive(Serialize)]
pub struct AuthRes {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct ErrMessage {
    pub code: Option<u8>,
    pub message: Option<String>,
}
