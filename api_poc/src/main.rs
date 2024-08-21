use actix_web::{HttpResponse, Responder};
mod api_endpoints;
mod app_config;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware, middleware::Logger, web, App, HttpServer};
    use env_logger::Env;
    use app_config::AppConfig;
    
    // Load configuration
    let config = AppConfig::build_config().expect("Failed to load configuration");

    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // actix will create a handler using ::new func. By default it matches the number of available cores.
    // this means the following code will be called n-times.
    HttpServer::new(move || {
        let api_scope = web::scope("/api")
            .configure(|svc_config| api_endpoints::api_config(svc_config, config.appname.clone()));

        App::new()
            // log each request. See https://docs.rs/actix-web/4.2.1/actix_web/middleware/struct.Logger.html#format
            // ex:
            // first line of request + response status + time take to serve request in ms
            // [2024-08-21T20:44:01Z INFO  actix_web::middleware::logger] POST /api/echo HTTP/1.1 200 1.491200
            .wrap(Logger::new("%r %s %D"))
            .wrap(middleware::Compress::default())
            .service(api_scope)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}