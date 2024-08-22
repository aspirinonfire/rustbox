use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    game::{license_plates::SpottedPlate, score_calculator::GameScoreResult},
    AppState,
};

// derive macro instructs compiler to implement Deserialize trait automatically.
// somewhat similar to dotnet source generators
#[derive(Deserialize)]
struct HelloParams {
    username: String,
}

#[derive(Serialize)]
struct EchoMessage<'a> {
    app_name: &'a str,
    request_body: Value,
}

#[get("/hello/{user_name}")]
async fn hello(data: web::Data<AppState>, hello_params: web::Path<HelloParams>) -> impl Responder {
    let app_name = &data.config.appname;
    // must use format!("{}", struct.field) rather than format!("{struct.field}")
    // because this is a macro and compiler will replace placeholder at the compile time.
    // this works differently than dotnet string interpolation
    let hello_message = format!("Hello world {} from {}", hello_params.username, app_name);
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
        .service(hello)
        .service(echo)
        .service(calc_score);
}
