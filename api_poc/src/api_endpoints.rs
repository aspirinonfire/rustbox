use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use log::info;
use serde::Serialize;
use serde_json::Value;

use crate::{
    auth::route_metadata::AppAuthParams,
    game::{license_plates::SpottedPlate, score_calculator::GameScoreResult},
    AppState,
};

#[derive(Serialize)]
struct EchoMessage<'a> {
    app_name: &'a str,
    request_body: Value,
}

#[get("/hello/{username}")]
async fn hello(
    data: web::Data<AppState>,
    username: web::Path<String>,
    auth_params: web::Data<AppAuthParams>,
) -> impl Responder {
    let app_name = &data.config.appname;
    // must use format!("{}", struct.field) rather than format!("{struct.field}")
    // because this is a macro and compiler will replace placeholder at the compile time.
    // this works differently than dotnet string interpolation
    let hello_message = format!(
        "Hello world {} from {}. Allow anon {}",
        username, app_name, auth_params.allow_anonymous
    );
    HttpResponse::Ok().json(hello_message)
}

#[post("/echo")]
async fn echo(req_body: web::Json<Value>, data: web::Data<AppState>) -> impl Responder {
    info!("Processing 'echo' request");

    let echo_msg = EchoMessage {
        app_name: &data.config.appname,
        request_body: req_body.into_inner(),
    };

    HttpResponse::Ok().json(echo_msg)
}

#[post("/calc_score")]
async fn calc_score(req_body: web::Json<Vec<SpottedPlate>>) -> impl Responder {
    let spotted_plates = req_body.into_inner();

    let game_score = GameScoreResult::new(&spotted_plates);

    HttpResponse::Ok().json(game_score)
}

/// Configure `/api` endpoints.
///
/// ### Params
/// `app_name`: Name of the application
pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
        // anonymous endpoints
        .service(
            web::scope("")
                // TODO replace it with request extensions
                .app_data(Data::new(AppAuthParams {
                    allow_anonymous: true,
                }))
                .service(hello),
        )
        // endpoints that will be authenticated by default
        .service(echo)
        .service(calc_score);
}
