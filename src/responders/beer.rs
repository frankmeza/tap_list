use crate::{handlers, models::ErrorResponse};

use actix_web::{HttpResponse, Responder};
// use actix_web::{web, HttpRequest, HttpResponse, Responder};
// use std::time::{SystemTime, UNIX_EPOCH};
// use uuid::Uuid;

pub async fn get_beer_list() -> impl Responder {
    let beer_list = handlers::fetch_beer_list();

    match beer_list.await {
        Err(error) => HttpResponse::ServiceUnavailable().json(ErrorResponse {
            message: format!("{:?}", error),
        }),
        Ok(beer_list) => HttpResponse::Ok().json(beer_list),
    }
}
