use std::{collections::HashMap, future::{ready, Ready}, sync::Arc};

use actix_web::{
    body::EitherBody, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, http::header::AUTHORIZATION, web::Data, Error, HttpMessage, HttpResponse
};
use futures_util::{future::LocalBoxFuture, FutureExt as _, TryFutureExt as _};
use log::{error, info};

use crate::AppState;

/// [Inspired by](https://github.com/actix/examples/blob/master/middleware/rate-limit/src/rate_limit.rs)
pub struct JwtAuthentication {
    // TODO add jwt params
}

#[derive(Debug, Clone)]
pub struct UserIdentityClaims(pub HashMap<String, String>);

/// JWT auth middleware
pub struct JwtAuthenticationMiddleware<S> {
    /// The next service to call after this one
    service: S,
}

/// JWT Auth middleware implementation
impl<S, B> Service<ServiceRequest> for JwtAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("Authenticating {}", req.uri());

        let app_state = req.app_data::<Data<Arc<AppState>>>();
        if app_state.is_none() {
            error!("AppState not found in app_data. TokenService is not available");
            return  Box::pin(async {
                Ok(req.into_response(HttpResponse::InternalServerError().finish().map_into_right_body()))
            });
        }

        // TODO check if route expects anonymous auth. Use allow-list for simplicity
        
        let bearer_token = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header_value|
                header_value
                    .to_str()
                    // TODO make Bearer case-insenstive
                    .map_or(None, |header_str| header_str.strip_prefix("Bearer "))
            );

        if bearer_token.is_none() {
            error!("Bearer token was not found in request headers");
            return  Box::pin(async {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
            });
        };

        // unwrapping is safe here because we have already validated app_state and bearer_token for None
        let claims = app_state.unwrap()
            .token_service.get_validated_claims(bearer_token.unwrap());

        match claims {
            Ok(claims) => {
                // add claims to request extensions so endpoints can use claims for further processing
                req.extensions_mut()
                    .insert(UserIdentityClaims(claims));

                self.service
                    .call(req)
                    .map_ok(ServiceResponse::map_into_left_body)
                    .boxed_local()
            },
            Err(err) => {
                error!("Bearer token is invalid: {err}");
                
                Box::pin(async {
                    Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
                })
            }
        }

    }
}

/// Jwt Auth middleware factory
impl<S, B> Transform<S, ServiceRequest> for JwtAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // Define associated types for the middleware
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error; // Type of error
    type Transform = JwtAuthenticationMiddleware<S>; // Type representing the transformed middleware
    type InitError = (); // Type of initialization error
    type Future = Ready<Result<Self::Transform, Self::InitError>>; // Type of the future returned by initialization

    // Initialize the middleware
    fn new_transform(&self, service: S) -> Self::Future {
        // Return a Ready future containing the JwtAuthenticationMiddleware instance
        ready(Ok(JwtAuthenticationMiddleware { service }))
    }
}
