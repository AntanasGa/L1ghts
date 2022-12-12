use crate::api::ApiError;
use crate::calls::AuthToken;
use crate::types::JWTAuthToken;
use dotenvy::dotenv;
use std::env;
use actix_web::{
    HttpRequest,
};
use jsonwebtoken::{
    decode,
    DecodingKey,
    Validation,
};

pub fn authorization(req: &HttpRequest) -> Result<JWTAuthToken, ApiError> {
    let headers = req.headers();
    if !headers.contains_key("Authorization") {
        return Err(ApiError::Forbidden);
    }
    let authorization = headers
        .get("Authorization")
        .ok_or(ApiError::InternalErr)
        ?.to_str()
        .map_err(|err| {
            log::error!("Failed to fetch auth: {}", err);
            ApiError::InternalErr
    })?.to_string();
    // removes [Bearer ]
    let auth_token = authorization.to_string().replace("Bearer ", "");
    let envy = dotenv();
    if envy.is_err() {
        log::error!("could not load env: {}", envy.unwrap_err());
        return Err(ApiError::InternalErr);
    }
    let secret = env::var("JWT_AUTH").map_err(|e| {
        log::error!("JWT_AUTH not found in env: {}", e);
        ApiError::InternalErr
    })?;
    let token = decode::<AuthToken>(&auth_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map_err(|err| {
            if err.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                return ApiError::Unauthorized;
            }
            log::error!("could not decode token: [{}], err: {}", auth_token, err);
            ApiError::Forbidden
    })?;
    Ok(token)
}
