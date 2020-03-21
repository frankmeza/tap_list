// use actix_web::{web, HttpRequest, HttpResponse, Responder};
// use std::time::{SystemTime, UNIX_EPOCH};
// use uuid::Uuid;

use crate::{
    handlers::{handle_beer_list, handle_beer_list_filtered_by},
    models::ErrorResponse,
    queries::generate_enum,
};

use actix_web::{web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterTypeAndValue {
    filter_type: String,
    filter_value: String,
}

pub async fn fetch_beer_list() -> impl Responder {
    let beer_list = handle_beer_list();

    match beer_list.await {
        Err(error) => HttpResponse::ServiceUnavailable().json(ErrorResponse {
            message: format!("ERROR: fetch_beer_list {:?}", error),
        }),
        Ok(beer_list) => HttpResponse::Ok().json(beer_list),
    }
}

pub async fn fetch_beers_filtered_by(f: web::Json<FilterTypeAndValue>) -> impl Responder {
    let ftv = FilterTypeAndValue {
        filter_type: String::from(&f.filter_type),
        filter_value: String::from(&f.filter_value),
    };

    let search_enum = generate_enum(ftv.filter_type);
    let beer_list = handle_beer_list_filtered_by(search_enum, &ftv.filter_value);

    match beer_list.await {
        Err(error) => HttpResponse::ServiceUnavailable().json(ErrorResponse {
            message: format!("ERROR: fetch_beers_filtered_by {:?}", error),
        }),
        Ok(beer_list) => HttpResponse::Ok().json(beer_list),
    }
}
