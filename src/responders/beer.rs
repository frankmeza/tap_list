use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn get_beer_list() -> impl Responder {
    HttpResponse::Ok()
}