use actix_web::{web, HttpResponse, Responder};

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