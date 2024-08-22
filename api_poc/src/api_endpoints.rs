use actix_web::{get, post, web, HttpResponse, Responder};
use serde:: { Serialize, Deserialize };
use serde_json::Value;
use log::info;

use crate::AppState;

// derive macro instructs compiler to implement Deserialize trait automatically.
// somewhat similar to dotnet source generators
#[derive(Deserialize)]
struct HelloParams {
    username: String
}

#[derive(Serialize)]
struct EchoMessage<'a> {
    app_name: &'a str,
    request_body: Value
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
      // must 'clone' the original app_name value because EchoMessage will outlive echo function
      // and therefore we can't use lifetimes in struct def.
      app_name: &data.config.appname,
      request_body: req_body.into_inner()
    };
    
    HttpResponse::Ok().json(echo_msg)
}

/// Configure `/api` endpoints.
/// 
/// ### Params
/// `app_name`: Name of the application
pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(hello)
        .service(echo);
}
