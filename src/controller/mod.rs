use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Puzzle {
    _id: ObjectId,
    puzzle_id: String,
    fen: String,
    moves: String,
    rating: i32,
    rating_deviation: i32,
    popularity: i32,
    nb_plays: i32,
    theme: String,
    game_url: String,
}

pub async fn get(
    app_data: web::Data<crate::AppState>,
) -> impl Responder {
    let result = web::block(move || app_data.service_container.puzzle.get()).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}