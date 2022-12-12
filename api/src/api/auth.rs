use crate::api::ApiError;
use crate::calls::*;
use crate::types::{
    Tokens,
    DbPool,
};
use crate::models::*;

use actix_web::{
    web,
    Responder,
    Result,
    // http::header,
    HttpRequest,
    HttpResponse,
};
use bcrypt::{ verify, /* hash, DEFAULT_COST */ };
use chrono::{ Utc, Duration };
use diesel::prelude::*;
use diesel::{insert_into, delete, update};
use jsonwebtoken::{ encode, /* decode, */ EncodingKey, Header };


pub async fn post(
    pool: web::Data<DbPool>,
    tokens: web::Data<Tokens>,
    auth_req: web::Json<AuthReq>,
    rq: HttpRequest
) -> Result<impl Responder, ApiError> {
    // wowie, thats a lot of work just for one header...
    let ua = rq.headers().get("user-agent")
    .ok_or(ApiError::InternalErr)
    ?.to_str()
    .map_err(|_| { ApiError::InternalErr })?;

    let uname = auth_req.user_name.to_string();
    // moving ownership is fun
    let pool_cred = pool.clone();
    let creds = web::block(move || {
        let mut con = pool_cred.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::credentials::dsl::*;
        credentials.filter(user_name.eq(uname))
        .load::<Credentials>(&mut con)
        .map_err(|err| {
            log::error!("Requesting Credentials failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Web block failed with: {}", err);
        ApiError::InternalErr
    // i guess this is a thing
    })??;
    // Empty string hashed (saving some cycles from hashing an empty string)
    let empty_hash = "$2y$12$nlS3Cp2Ehb7C.InFZJgryeXMWJR6j4dW1kZiWF1K535ZAlaEhLSIO".to_string();
    let mut pass_hash = empty_hash.to_string();
    if creds.len() == 1 {
        let check = &creds[0].pass;
        pass_hash = match check {
            Some(v) => v.clone(),
            None => pass_hash.to_string(),
        };
    }
    let req_pass = auth_req.password.to_string();
    let verify_pass_hash = pass_hash.to_string();
    let valid = web::block(move || {
        verify(&req_pass, &verify_pass_hash.to_string()).map_err(|_| {
            log::error!("Failed to verify password for {}", auth_req.user_name);
            ApiError::InternalErr
        })
    }).await
    .map_err(|err| {
        log::error!("Web block failed with: {}", err);
        ApiError::InternalErr
    })??;
    if pass_hash == empty_hash || !valid {
        return Err(ApiError::Unauthorized);
    }

    let now = Utc::now();
    let expires = now + Duration::seconds(15);
    // let expires = now + Duration::minutes(15);

    let uid = *&creds[0].id.clone();
    let auth_token = AuthToken {
        uid,
        exp: expires.timestamp(),
    };

    let access = encode(&Header::default(), &auth_token, &EncodingKey::from_secret(tokens.auth.as_ref()));
    let refresh_token = RefreshToken {
        uid,
        created_at: now.timestamp_millis(),
    };
    let refresh = encode(&Header::default(), &refresh_token, &EncodingKey::from_secret(tokens.refresh.as_ref()));
    if access.is_err() || refresh.is_err() {
        log::error!("failed to encode jwt keys");
        return Err(ApiError::InternalErr);
    }
    let refresh_token = refresh.unwrap();
    let new_refresh = NewCredentialRefresh {
        credential_id: uid,
        token: refresh_token.clone(),
        user_agent: ua.to_string(),
        created_at: now.naive_utc(),
        used_at: now.naive_utc(),
    };
    let pool_refr = pool.clone();
    web::block(move || {
        let mut con = pool_refr.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::credential_refresh::dsl::*;
        insert_into(credential_refresh)
        .values(&new_refresh)
        .execute(&mut con).map_err(|err| {
            log::error!("Could not insert new refresh token: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Web block failed with: {}", err);
        ApiError::InternalErr
    })??;
    let res = AuthRes {
        access_token: access.unwrap(),
        refresh_token: refresh_token.clone(),
    };
    Ok(web::Json(res))
}

pub async fn refresh_post(
    pool: web::Data<DbPool>,
    tokens: web::Data<Tokens>,
    req: web::Json<Refresh>,
    rq: HttpRequest
) -> Result<impl Responder, ApiError> {
    let tok = req.token.to_string();
    use crate::schema::credential_refresh::dsl::*;
    let pool_refr = pool.clone();
    let pool_tok = tok.clone();
    let refresh_items = web::block(move || {
        let mut con = pool_refr.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool: {}", err);
            ApiError::InternalErr
        })?;
        credential_refresh
        .filter(token.eq(pool_tok))
        .load::<CredentialRefresh>(&mut con)
        .map_err(|err| {
            log::error!("Requesting Data failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Web block failed with: {}", err);
        ApiError::InternalErr
    })??;
    if refresh_items.len() == 0 {
        return Err(ApiError::Unauthorized);
    }
    // bOrRoWeD VaLuE DoEs nOt lIvE LoNg eNoUgHrUsTc(e0597)
    let matched = refresh_items[0].clone();

    if refresh_items.len() > 1 {
        // Should never happen, but is here just in case
        let pool_del = pool.clone();
        web::block(move || {
            let mut con = pool_del.get()
            .map_err(|err| {
                log::error!("Could not fetch connection from pool: {}", err);
                ApiError::InternalErr
            })?;
            match delete(credential_refresh.filter(token.eq(tok.to_string())))
            .execute(&mut con) {
                Ok(val) => Ok(val),
                Err(e) => {
                    log::error!("Refresh token cleanup failed: [{}] with: {}", tok.to_string(), e);
                    Err(ApiError::InternalErr)
                }
            }
        })
        .await
        .map_err(|err| {
            log::error!("Web block failed with: {}", err);
            ApiError::InternalErr
        })??;
        return Err(ApiError::InternalErr);
    }
    let ua = rq.headers()
    .get("user-agent")
    .ok_or(ApiError::InternalErr)
    ?.to_str()
    .map_err(|_| { ApiError::InternalErr })
    ?.to_string();
    if matched.user_agent != ua {
        return Err(ApiError::Unauthorized);
    }

    let now = Utc::now();
    // let expires = now + Duration::minutes(15);
    let expires = now + Duration::seconds(15);

    let auth_token = AuthToken {
        uid: matched.credential_id,
        exp: expires.timestamp(),
    };
    let access = encode(&Header::default(), &auth_token, &EncodingKey::from_secret(tokens.auth.as_ref()));
    if access.is_err() {
        log::error!("encoding refresh token failed: {}", access.unwrap_err());
        return Err(ApiError::InternalErr);
    }
    let pool_update = pool.clone();
    web::block(move || {
        let mut con = pool_update.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool: {}", err);
            ApiError::InternalErr
        })?;
        match update(credential_refresh.find(matched.id))
        .set(used_at.eq(now.naive_utc()))
        .execute(&mut con) {
            Ok(val) => Ok(val),
            Err(e) => {
                log::error!("Failed to update refresh token for credentials_id: [{}] with: {}", matched.credential_id, e);
                Err(ApiError::InternalErr)
            }
        }
    })
    .await
    .map_err(|err| {
        log::error!("Web block failed with: {}", err);
        ApiError::InternalErr
    })??;

    let res = Refresh {
        token: access.unwrap(),
    };

    Ok(web::Json(res))
}

pub async fn refresh_delete(
    pool: web::Data<DbPool>,
    req: web::Json<Refresh>,
    rq: HttpRequest
) -> Result<impl Responder, ApiError> {
    use crate::schema::credential_refresh::dsl::*;
    let tok = req.token.to_string();
    let pool_find = pool.clone();
    let refresh_items = web::block(move || {
        let mut con = pool_find.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool: {}", err);
            ApiError::InternalErr
        })?;
        credential_refresh
        .filter(token.eq(tok))
        .load::<CredentialRefresh>(&mut con)
        .map_err(|err| {
            log::error!("Fetching refresh token failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Web block failed with: {}", err);
        ApiError::InternalErr
    })??;
    if refresh_items.len() == 0 {
        log::error!("Could not find the token");
        return Err(ApiError::Unauthorized);
    }
    let matched = refresh_items[0].clone();
    if refresh_items.len() > 1 {
        // Should never happen, but is here just in case
        let pool_overflow = pool.clone();
        let overflow_token = req.token.to_string();
        web::block(move || {
            let mut con = pool_overflow.get()
            .map_err(|err| {
                log::error!("Could not fetch connection from pool: {}", err);
                ApiError::InternalErr
            })?;
            match delete(credential_refresh.filter(token.eq(overflow_token)))
            .execute(&mut con) {
                Ok(v) => Ok(v),
                Err(err) => {
                    log::error!("Failed to clear credential overflow: {}", err);
                    Err(ApiError::InternalErr)
                },
            }
        })
        .await
        .map_err(|err| {
            log::error!("Web block failed with: {}", err);
            ApiError::InternalErr
        })??;
        return Err(ApiError::InternalErr);
    }
    let ua = rq.headers()
        .get("user-agent")
        .ok_or(ApiError::InternalErr)
        ?.to_str()
        .map_err(|_| { ApiError::InternalErr })
        ?.to_string();

    if matched.user_agent.to_string() != ua.to_string() {
        log::error!("User agents do not match, [{}] [{}]", matched.user_agent.to_string(), ua);
        return Err(ApiError::Unauthorized);
    }
    let pool_del = pool.clone();
    web::block(move || {
        let mut con = pool_del.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool: {}", err);
            ApiError::InternalErr
        })?;
        match delete(credential_refresh.filter(id.eq(matched.id.clone())))
        .execute(&mut con) {
            Ok(v) => Ok(v),
            Err(e) => {
                log::error!("Could delete refresh item id: {}, err: {}", matched.id.clone(), e);
                Err(ApiError::InternalErr)
            },

        }
    })
    .await
    .map_err(|err| {
        log::error!("Web block failed with: {}", err);
        ApiError::InternalErr
    })??;
    Ok(HttpResponse::Ok())
}
