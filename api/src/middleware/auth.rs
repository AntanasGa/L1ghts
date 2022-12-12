use crate::api::{
    helpers::auth::authorization,
};
use crate::types::JWTAuthToken;

use std::future::{ready, Ready};

use std::rc::Rc;

use actix_web::body::EitherBody;
// use actix_web::error::ErrorProxyAuthenticationRequired;
// use actix_web::{HttpResponse, web};
// use actix_web::http::Method;
use actix_web::{
    dev::{
        forward_ready,
        Service,
        ServiceRequest,
        ServiceResponse,
        Transform,
    },
    HttpMessage,
    Error,
};
use futures_util::future::LocalBoxFuture;

pub type TokenData = Rc<JWTAuthToken>;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct TokenFactory;

impl TokenFactory {
    pub fn new() -> Self {
        TokenFactory {}
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for TokenFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = TokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenMiddleware { service: Rc::new(service) }))
    }
}

pub struct TokenMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        Box::pin(async move {
            match authorization(req.request()) {
                Ok(v) => {
                    req.extensions_mut().insert::<TokenData>(Rc::new(v));
                    let beep = srv.call(req)
                    .await
                    .map_err(|err| {
                        log::error!("middleware srv call error: {}", err);
                        err
                    })?;
                    Ok(beep.map_into_left_body())
                },
                Err(e) => {
                    Ok(req.error_response(e).map_into_right_body())
                }
            }
        })
    }
}