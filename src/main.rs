use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use service::PuzzleService;
use std::env;

mod controller;
mod service;

// service container
pub struct ServiceContainer {
    puzzle: PuzzleService,
}

impl ServiceContainer {
    pub fn new(puzzle: PuzzleService) -> Self {
        ServiceContainer { puzzle }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

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
    let client = mongodb::Client::with_uri_str(env::var("CHESS_MONGODB_URI").unwrap()).await;
    match client {
        Ok(client) => {
            let db = client.database("chess-puzzles");
            let collection = db.collection("puzzles");

            HttpServer::new(move || {
                let service_container =
                    ServiceContainer::new(PuzzleService::new(collection.clone()));
                App::new()
                    .data(AppState { service_container })
                    .route("/", web::get().to(welcome))
                    .route("/api/v1/get-puzzle", web::get().to(controller::get_puzzle))
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await
        }
        // Convert mongodb::error::Error into std::io::Error
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}
