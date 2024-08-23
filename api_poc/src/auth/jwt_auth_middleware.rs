use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use log::info;

use super::route_metadata::AppAuthParams;

/// [Inspired by](https://medium.com/@sarathraj2008/middleware-in-actix-rust-5f7b9860ac70)
pub struct JwtAuthentication {
    // TODO add jwt params
}

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
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("Authenticating {}", req.uri());

        // check if request allows anon
        // TODO replace with request extensions
        let route_auth_params = req.app_data::<Data<AppAuthParams>>();

        match route_auth_params {
            Some(state) => info!("allow auth {}", state.allow_anonymous),
            None => info!("auth is missing"),
        }

        let allow_anonymous =
            route_auth_params.map_or(false, |auth_params| auth_params.allow_anonymous);

        if allow_anonymous {
            info!(
                "Current endpoint allows anonymous authentication. Proceeding to next handler..."
            );
            let future = self.service.call(req);
            return Box::pin(future);
        }

        // TODO extract bearer token from authorization header
        // TODO validate bearer token as jwt, short-circuit with 401 if missing or invalid
        //let headers = req.headers();

        let future = self.service.call(req);
        Box::pin(future)
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
    type Response = ServiceResponse<B>; // Type of the response
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
