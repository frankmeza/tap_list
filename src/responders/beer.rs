use crate::{handlers::handle_beer_list, models::ErrorResponse};

use actix_web::{HttpResponse, Responder};
// use actix_web::{web, HttpRequest, HttpResponse, Responder};
// use std::time::{SystemTime, UNIX_EPOCH};
// use uuid::Uuid;

// struct RequestId {
//     id:
// }

pub async fn fetch_beer_list() -> impl Responder {
    let beer_list = handle_beer_list();

    match beer_list.await {
        Err(error) => HttpResponse::ServiceUnavailable().json(ErrorResponse {
            message: format!("{:?}", error),
        }),
        Ok(beer_list) => HttpResponse::Ok().json(beer_list),
    }
}

// pub async fn fetch_beers_filtered_by(web::Json<RequestId>) ->