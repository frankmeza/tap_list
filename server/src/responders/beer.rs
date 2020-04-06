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
use tokio_postgres::error::Error as TokioPgError;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterTypeAndValue {
    filter_type: String,
    filter_value: String,
}

pub async fn fetch_beer_list() -> impl Responder {
    let beer_list = handle_beer_list();

    match beer_list.await {
        Err(error) => service_unavailable("fetch_beer_list", error),
        Ok(beer_list) => HttpResponse::Ok().json(beer_list),
    }
}

pub async fn fetch_beers_filtered_by(
    f: web::Json<FilterTypeAndValue>,
) -> impl Responder {
    let ftv = FilterTypeAndValue {
        filter_type: String::from(&f.filter_type),
        filter_value: String::from(&f.filter_value),
    };

    let search_enum = generate_enum(ftv.filter_type);

    let beer_list =
        handle_beer_list_filtered_by(search_enum, &ftv.filter_value);

    match beer_list.await {
        Err(error) => service_unavailable("fetch_beers_filtered_by", error),
        Ok(beer_list) => HttpResponse::Ok().json(beer_list),
    }
}

fn service_unavailable(fn_name: &str, error: TokioPgError) -> HttpResponse {
    HttpResponse::ServiceUnavailable().json(ErrorResponse {
        message: format!("ERROR: {} {:?}", fn_name, error),
    })
}
