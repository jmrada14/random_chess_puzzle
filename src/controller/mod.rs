use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct Puzzle {
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

pub async fn get_puzzle(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = web::block(move || app_data.service_container.puzzle.get_puzzle()).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
