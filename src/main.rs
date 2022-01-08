use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use service::PuzzleService;

#[macro_use]
extern crate dotenv_codegen;

mod controller;
mod service;

pub struct ServiceContainer {
    puzzle_service: PuzzleService,
}

impl ServiceContainer {
    pub fn new(puzzle_service: PuzzleService) -> Self {
        ServiceContainer { puzzle_service }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse(dotenv!("DB")).unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("chess-puzzles");

    let puzzle_collection = db.collection("puzzles");

    HttpServer::new(move || {
        let service_container = ServiceContainer::new(PuzzleService::new(puzzle_collection.clone()));

        App::new()
            .data(AppState { service_container })
            .route("/get", web::get().to(controller::get))
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}