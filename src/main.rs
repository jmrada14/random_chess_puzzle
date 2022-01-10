use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use service::PuzzleService;
use mongodb::results::Resu

mod service;

//Root handler
async fn welcome() -> impl Responder {
    HttpResponse::Ok().json(json!(
        {
            "Hello": "Welcome to the Rust random chess puzzle API",
            "Endpoints": [
            "/api/v1/get-puzzle", "api/v1/get-puzzles:/number_of_puzzles"
            ]
        }
    ))
}


// Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(welcome)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
