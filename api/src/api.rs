pub mod helpers;
mod auth;
mod devices;
mod points;
mod presets;
mod setup;

use crate::{
    middleware::auth::TokenFactory,
};
use actix_web::{
    web,
    Scope,
    HttpResponse,
    ResponseError,
    http::{
        header::ContentType,
        StatusCode,
    },
};
use serde::Serialize;
use derive_more::Display;
use serde_json;



pub fn expose_api() -> Scope {
    web::scope("/api")
        .route("", web::get().to(|| async { "OK" }))
        .service(
            web::scope("/step")
            .service(
                web::resource("")
                .route(web::get().to(self::setup::get))
                .route(web::post().to(self::setup::post))
            )
        )
        .service(
            web::scope("/auth")
            .service(
                web::resource("")
                .route(web::post().to(self::auth::post))
                .route(web::delete().to(self::auth::refresh_delete))
            )
            .route("/refresh", web::post().to(self::auth::refresh_post))
        )
        .service(
            web::scope("/devices")
            .wrap(TokenFactory::new())
            .service(
                web::resource("")
                .route(web::get().to(self::devices::get))
                .route(web::post().to(self::devices::post))
            )
        )
        .service(
            web::scope("/points")
            .wrap(TokenFactory::new())
            .service(
                web::resource("")
                .route(web::get().to(self::points::get))
                .route(web::put().to(self::points::put))
            )
            .service(
                web::resource("/identify")
                .route(web::post().to(self::points::identify::post))
            )
            .service(
                web::resource("/single/{point}")
                .route(web::get().to(self::points::single::get))
                .route(web::put().to(self::points::single::put))
            )
        )
        .service(
            web::scope("/presets")
            .wrap(TokenFactory::new())
            .service(
                web::resource("")
                .route(web::get().to(self::presets::get))
                .route(web::post().to(self::presets::post))
                .route(web::put().to(self::presets::upd))
                .route(web::delete().to(self::presets::del))
            )
            .service(
                web::resource("/active")
                .route(web::get().to(self::presets::active::get))
                .route(web::put().to(self::presets::active::put))
            )
            .service(
                web::resource("/points")
                .route(web::put().to(self::presets::points::put))
            )
        )
}

#[derive(Debug, Display)]
pub enum ApiError {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    TooManyRequests,
    InternalErr,
    NotImplemented,
    Unavailable,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        #[derive(Debug, Serialize)]
        struct ErrMessage {
            pub message: String,
        }
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&ErrMessage { message: self.to_string() }).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict => StatusCode::CONFLICT,
            ApiError::InternalErr => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ApiError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}
