use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;

use crate::service::Puzzle;

pub async fn get_puzzle(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = app_data.service_container.puzzle.get_puzzle().await;
    match result {
        Ok(Some(result)) => HttpResponse::Ok().json(result),
        Ok(None) => {
            println!("No puzzle found");
            HttpResponse::InternalServerError().finish()
        }
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
