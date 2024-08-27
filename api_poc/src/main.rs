use std::sync::Arc;

use app_config::AppConfig;
use auth::{
    jwt_auth_middleware::JwtAuthentication,
    token_service::{JwtTokenService, TokenService},
};
use game::player::Player;
use log::info;

mod api_endpoints;
mod app_config;
mod auth;
mod game;

struct AppState {
    config: AppConfig,
    /// ## TokenService [trait object](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
    /// `Box<dyn ...>` enables a dynamic dispatch (vtable equivalent)
    /// allowing token service implementation to be known at the runtime rather than compile time.
    /// This is not strictly necessary for this project.
    token_service: Box<dyn TokenService>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware, middleware::Logger, web, App, HttpServer};
    use app_config::AppConfig;
    use env_logger::Env;
    use mongodb::Client;

    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration
    info!("reading configuration...");

    let config = AppConfig::build_config().expect("Failed to load configuration");
    let bind_host = (config.host_ip.clone(), config.port);

    info!("attempting to connect to mongo...");
    let game_api_mongo_db = Client::with_uri_str(&config.mongo_connection_string)
        .await
        .expect("Failed to connect mongo client")
        .database("game_api");

    let _ = Player::create_identity_index(&game_api_mongo_db)
        .await
        .expect("Failed to create player index");

    info!("mongo connected. initializing api handlers");

    let app_state = Arc::new(AppState {
        token_service: Box::new(JwtTokenService::new(
            &config.jwt_signing_key,
            &config.appname,
            &config.appname,
            1,
            config.token_lifetime_min,
        )),
        config,
    });

    let game_api_mongo_db = Arc::new(game_api_mongo_db);

    // actix will call this function for the requested number of handlers (default == num of cores)
    HttpServer::new(move || {
        let api_scope = web::scope("/api").configure(api_endpoints::api_config);

        App::new()
            // middleware is executed in LIFO (stack) order
            .wrap(middleware::Compress::default())
            .wrap(JwtAuthentication::new(vec!["/api/token".into()])) // must be wrapped first to avoid compilation errors
            // log each request. See https://docs.rs/actix-web/4.2.1/actix_web/middleware/struct.Logger.html#format
            // ex:
            // first line of request + response status + time take to serve request in ms
            // [2024-08-21T20:44:01Z INFO  actix_web::middleware::logger] POST /api/echo HTTP/1.1 200 1.491200ms
            .wrap(Logger::new("%r %s %Dms"))
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(game_api_mongo_db.clone()))
            .service(api_scope)
    })
    .bind(bind_host)
    .expect("Address and port should be free and valid")
    .run()
    .await
}
