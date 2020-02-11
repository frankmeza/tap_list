use crate::{get_connection, handlers};

use actix_web::{HttpResponse, Responder};
// use actix_web::{web, HttpRequest, HttpResponse, Responder};
// use std::time::{SystemTime, UNIX_EPOCH};
// use uuid::Uuid;

pub fn get_beer_list() -> impl Responder {
    let conn = get_connection();
    let beer_list = handlers::fetch_beer_list(conn);

    HttpResponse::Ok().json(beer_list)
}
