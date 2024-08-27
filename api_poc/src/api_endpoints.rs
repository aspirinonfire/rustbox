use std::sync::Arc;

use actix_web::{
    get, post,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use bson::doc;
use log::info;
use mongodb::Database;

use crate::{
    auth::jwt_auth_middleware::UserIdentityClaims,
    game::{license_plates::SpottedPlate, score_calculator::GameScoreResult},
    AppState,
};

#[get("/hello/{name}")]
async fn hello(
    data: web::Data<Arc<AppState>>,
    db: web::Data<Arc<Database>>,
    name: web::Path<String>,
    claims: ReqData<UserIdentityClaims>,
) -> impl Responder {
    let app_name = &data.config.appname;

    let this_user_id = match claims.0.get("sub") {
        Some(v) => v,
        None => "n/a",
    };

    let command = doc! {
        "find": "dummy_collection", // This collection does not need to exist
        "filter": {},               // No filter, just a placeholder
        "projection": { "echo": "hello world!" }, // Projection with a made-up field
        "limit": 1                  // Limit to 1 result for simplicity
    };

    let db_result = db.run_command(command)
        .await
        .unwrap();

    let hello_message = format!("Hello {name} from {this_user_id} and {app_name}.");

    let response = (hello_message, db_result);

    HttpResponse::Ok().json(response)
}

#[post("/calc_score")]
async fn calc_score(req_body: web::Json<Vec<SpottedPlate>>) -> impl Responder {
    info!("Calculating score...");
    let spotted_plates = req_body.into_inner();

    let game_score = GameScoreResult::new(&spotted_plates);

    HttpResponse::Ok().json(game_score)
}

/// Configure `/api` endpoints.
pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello).service(calc_score);
}
