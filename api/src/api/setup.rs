use actix_web::{
    web,
    Responder,
    HttpResponse,
};
use bcrypt::{
    hash,
    DEFAULT_COST,
};

use crate::{
    types::{
        DbPool,
        SharedStorage
    },
    calls::{
        StepGetRes,
        SetupAuth,
    },
    models::{Credentials, NewCredentials},
};

use super::ApiError;

use diesel::{
    prelude::*,
    insert_into,
};



pub async fn get(pool: web::Data<DbPool>) -> Result<web::Json<StepGetRes>, ApiError> {
    let step = web::block(move || {
        let mut con = pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::credentials::dsl::*;
        let users = credentials.load::<Credentials>(&mut con)
        .map_err(|err| {
            log::error!("failed fetching users: {}", err);
            ApiError::InternalErr
        })?;
        match users.len() < 1 {
            true => Ok("setup"),
            false => Ok("installed"),
        }
    })
    .await
    .map_err(|err| {
        log::error!("Identify block failed: {}", err);
        ApiError::InternalErr
    })??;
    let res = StepGetRes {
        step: step.to_string()
    };
    Ok(web::Json(res))
}

pub async fn post(request: web::Json<SetupAuth>, pool: web::Data<DbPool>, shared_data: web::Data<SharedStorage>) -> Result<impl Responder, ApiError> {
    web::block(move || {
        let setup_secret = shared_data.setup_secret.clone().to_string();
        if request.key != setup_secret {
            return Err(ApiError::Unauthorized);
        }

        let mut con = pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;

        use crate::schema::credentials::dsl::*;
        let users = credentials.load::<Credentials>(&mut con)
        .map_err(|err| {
            log::error!("failed fetching users: {}", err);
            ApiError::InternalErr
        })?;
        if users.len() > 0 {
            return Err(ApiError::Forbidden);
        }

        let mut identity = request.user.clone();
        identity.user_name = identity.user_name.trim().to_string();
        identity.password = identity.password.trim().to_string();

        if identity.password.len() < 8 || identity.user_name.len() < 8 {
            return Err(ApiError::BadRequest);
        }

        identity.password = hash(identity.password, DEFAULT_COST)
        .map_err(|err| {
            log::error!("Failed to set user password: {}", err);
            ApiError::InternalErr
        })?;
        insert_into(credentials).values(
            NewCredentials {
                user_name: identity.user_name,
                pass: Some(identity.password),
                recovery_expires: None,
                recovery_key: None,
             }
        )
        .execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to insert user to databaste: {}", err);
            ApiError::InternalErr
        })?;
        Ok(())
    })
    .await
    .map_err(|err| {
        log::error!("Identify block failed: {}", err);
        ApiError::InternalErr
    })??;
    Ok(HttpResponse::Ok())
}