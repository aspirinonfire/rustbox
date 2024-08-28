use std::{
    collections::HashSet,
    future::{ready, Ready},
    sync::Arc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    web::Data,
    Error, HttpMessage, HttpResponse,
};
use futures_util::{future::LocalBoxFuture, FutureExt as _, TryFutureExt as _};
use log::{error, info};

use crate::AppState;

/// [Inspired by](https://github.com/actix/examples/blob/master/middleware/rate-limit/src/rate_limit.rs)
pub struct JwtAuthentication {
    anonymous_urls: Vec<String>,
}

impl JwtAuthentication {
    pub fn new(anonymous_urls: Vec<String>) -> Self {
        Self { anonymous_urls }
    }
}

/// JWT auth middleware
pub struct JwtAuthenticationMiddleware<S> {
    /// The next service to call after this one
    service: S,
    anonymous_urls: HashSet<String>,
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
        let request_url = &req.uri();
        info!("Authenticating {}", request_url);

        // check if route expects anonymous auth. Use allow-list for simplicity
        if self.anonymous_urls.contains(&request_url.to_string()) {
            info!("Uri is marked for anonymous auth. Skipping auth validation...");
            return self
                .service
                .call(req)
                .map_ok(ServiceResponse::map_into_left_body)
                .boxed_local();
        }

        let app_state = req.app_data::<Data<Arc<AppState>>>();
        if app_state.is_none() {
            error!("AppState not found in app_data. TokenService is not available");
            return Box::pin(async {
                Ok(req.into_response(
                    HttpResponse::InternalServerError()
                        .finish()
                        .map_into_right_body(),
                ))
            });
        }

        let bearer_token = req.headers().get(AUTHORIZATION).and_then(|header_value| {
            header_value
                .to_str()
                // TODO make Bearer case-insenstive
                .map_or(None, |header_str| header_str.strip_prefix("Bearer "))
        });

        if bearer_token.is_none() {
            error!("Bearer token was not found in request headers");
            return Box::pin(async {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
            });
        };

        // unwrapping is safe here because we have already validated app_state and bearer_token for None
        let claims = app_state
            .unwrap()
            .token_service
            .get_validated_claims(bearer_token.unwrap());

        match claims {
            Ok(claims) => {
                // add claims to request extensions so endpoints can use them to establish user context
                // for the time being, we don't need to query db because everything we'll need will be
                // stored in claims
                req.extensions_mut().insert(claims);

                self.service
                    .call(req)
                    .map_ok(ServiceResponse::map_into_left_body)
                    .boxed_local()
            }
            Err(err) => {
                error!("Bearer token is invalid: {err}");

                Box::pin(async {
                    Ok(req
                        .into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
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
        ready(Ok(JwtAuthenticationMiddleware {
            service,
            anonymous_urls: self.anonymous_urls.iter().cloned().collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{app_config::AppConfig, auth::token_service::JwtTokenService};

    use super::*;
    use actix_web::{
        http::{self, StatusCode},
        test, web, App,
    };

    #[actix_web::test]
    async fn will_return_401_on_missing_auth() {
        let app_state = Arc::new(AppState {
            token_service: Box::new(JwtTokenService::new("test key", "issuer", "audience", 1, 1)),
            config: AppConfig::default(),
        });

        let uut_app = test::init_service(
            App::new()
                .wrap(JwtAuthentication::new(vec!["".into()]))
                .app_data(web::Data::new(app_state.clone()))
                .route("/", web::get().to(HttpResponse::Ok)),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();

        let actual_resp = test::call_service(&uut_app, req).await;

        assert_eq!(StatusCode::UNAUTHORIZED, actual_resp.status());
    }

    #[actix_web::test]
    async fn will_return_401_on_bad_auth() {
        let app_state = Arc::new(AppState {
            token_service: Box::new(JwtTokenService::new("test key", "issuer", "audience", 1, 1)),
            config: AppConfig::default(),
        });

        let uut_app = test::init_service(
            App::new()
                .wrap(JwtAuthentication::new(vec!["".into()]))
                .app_data(web::Data::new(app_state.clone()))
                .route("/", web::get().to(HttpResponse::Ok)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/")
            .insert_header((http::header::AUTHORIZATION, "Bearer bad_token"))
            .to_request();

        let actual_resp = test::call_service(&uut_app, req).await;

        assert_eq!(StatusCode::UNAUTHORIZED, actual_resp.status());
    }

    #[actix_web::test]
    async fn will_return_200_on_valid_auth() {
        let app_state = Arc::new(AppState {
            token_service: Box::new(JwtTokenService::new("test key", "issuer", "audience", 1, 1)),
            config: AppConfig::default(),
        });

        let valid_token = app_state
            .token_service
            .generate_token("test_subject")
            .unwrap();

        let uut_app = test::init_service(
            App::new()
                .wrap(JwtAuthentication::new(vec!["".into()]))
                .app_data(web::Data::new(app_state.clone()))
                .route("/", web::get().to(HttpResponse::Ok)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/")
            // current POC implementation assumes valid token to match jwt audience
            .insert_header((
                http::header::AUTHORIZATION,
                format!("Bearer {}", &valid_token.access_token),
            ))
            .to_request();

        let actual_resp = test::call_service(&uut_app, req).await;

        assert_eq!(StatusCode::OK, actual_resp.status());
    }

    #[actix_web::test]
    async fn will_return_200_on_anonymous_auth() {
        let app_state = Arc::new(AppState {
            token_service: Box::new(JwtTokenService::new("test key", "issuer", "audience", 1, 1)),
            config: AppConfig::default(),
        });

        let uut_app = test::init_service(
            App::new()
                .wrap(JwtAuthentication::new(vec!["/test".into()]))
                .app_data(web::Data::new(app_state.clone()))
                .route("/test", web::get().to(HttpResponse::Ok)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/test")
            // current POC implementation assumes valid token to match jwt audience
            .to_request();

        let actual_resp = test::call_service(&uut_app, req).await;

        assert_eq!(StatusCode::OK, actual_resp.status());
    }
}
