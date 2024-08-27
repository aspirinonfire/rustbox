use std::sync::Arc;

use actix_web::{
    get, post,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use bson::doc;
use log::{error, info};
use mongodb::Database;

use crate::{
    auth::token_service::UserClaims,
    game::{license_plates::SpottedPlate, score_calculator::GameScoreResult},
    AppState,
};

#[get("/hello/{name}")]
async fn hello(
    data: web::Data<Arc<AppState>>,
    db: web::Data<Arc<Database>>,
    name: web::Path<String>,
    // TODO need fixing
    claims: ReqData<&UserClaims>,
) -> impl Responder {
    let app_name = &data.config.appname;

    let this_user_id = &claims.sub;

    let command = doc! {
        "find": "dummy_collection", // This collection does not need to exist
        "filter": {},               // No filter, just a placeholder
        "projection": { "echo": "hello world!" }, // Projection with a made-up field
        "limit": 1                  // Limit to 1 result for simplicity
    };

    let db_result = db.run_command(command).await.unwrap();

    let hello_message = format!("Hello {name} from {this_user_id} and {app_name}.");

    let response = (hello_message, db_result);

    HttpResponse::Ok().json(response)
}

/// Generate access token
/// For the purposes of POC this endpoint will accept and validate a subject string in memory.
///
/// Actual implementation will use Google authorization code to validate the identity,
/// create (or retrieve if exist) a player record from db, and only then generate API access token.
#[post("/token")]
async fn generate_token(
    req_body: web::Json<String>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let subject = req_body.into_inner();

    // validate request
    if data.config.allowed_subj != subject {
        error!("subject is not authorized!");
        return HttpResponse::Unauthorized().finish();
    }

    let token_result = data.token_service.generate_token(&subject);

    match token_result {
        Ok(token) => HttpResponse::Ok().json(token.token_value),
        Err(err) => {
            error!("failed to generate token {}", err);
            HttpResponse::Unauthorized().finish()
        }
    }
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
    cfg.service(hello)
        .service(calc_score)
        .service(generate_token);
}
