use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::stream::TryStreamExt;
use mongodb::bson::{self, doc, Bson};
use mongodb::options::{ClientOptions, ResolverConfig};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;


#[derive(Debug, Serialize, Deserialize)]
struct Puzzle {
    puzzle_id: String,
    fen: String,
    moves: String,
    rating: i32,
    rating_deviation: i32,
    popularity: i32,
    n_plays: i32,
    themes: String,
    game_url: String,
}

async fn get_puzzle() -> Response {
    let client_uri = env::var("CHESS_MONGODB_URI")
        .expect("You must set CHESS_MONGODB_URI environment variable!🐱");
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = mongodb::Client::with_options(options)?;
    let db = client.database("chess-puzzles");
    let collection = db.collection("puzzles");
    let mut cursor = collection.find(None, None).await?;
    let mut puzzles: Vec<Puzzle> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        let puzzle: Puzzle = bson::from_bson(doc)?;
        puzzles.push(puzzle);
    }
    let json = serde_json::to_string(&puzzles).unwrap();
    HttpResponse::Ok().body(json)
}