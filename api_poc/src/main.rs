use std::sync::Arc;

mod api_endpoints;
mod app_config;
mod auth;
mod game;

struct AppState {
    config: Arc<app_config::AppConfig>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware, middleware::Logger, web, App, HttpServer};
    use app_config::AppConfig;
    use auth::jwt_auth_middleware::JwtAuthentication;
    use env_logger::Env;

    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration
    let config = AppConfig::build_config().expect("Failed to load configuration");
    let bind_host = (config.host_ip.clone(), config.port);
    let config = Arc::new(config);

    // actix will call this function for the requested number of handlers (default == num of cores)
    HttpServer::new(move || {
        let api_scope = web::scope("/api").configure(api_endpoints::api_config);

        App::new()
            // middleware is executed in LIFO (stack) order
            .wrap(JwtAuthentication {})
            .wrap(middleware::Compress::default())
            // log each request. See https://docs.rs/actix-web/4.2.1/actix_web/middleware/struct.Logger.html#format
            // ex:
            // first line of request + response status + time take to serve request in ms
            // [2024-08-21T20:44:01Z INFO  actix_web::middleware::logger] POST /api/echo HTTP/1.1 200 1.491200ms
            .wrap(Logger::new("%r %s %Dms"))
            .app_data(web::Data::new(AppState {
                config: config.clone(),
            }))
            .service(api_scope)
    })
    .bind(bind_host)
    .expect("Address and port should be free and valid")
    .run()
    .await
}
