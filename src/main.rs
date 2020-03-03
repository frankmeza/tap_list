use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use postgres::{Client, NoTls as PgNoTls};
use tokio;
use tokio_postgres::{Error, NoTls, Row};

mod handlers;
mod models;
mod queries;
mod responders;
// mod ws_server;

extern crate env_logger;
extern crate ws;

async fn get_async_connection(query_string: String) -> Result<Vec<Row>, Error> {
    let connection_url = "postgres://postgres@localhost:5432/beer_tap_list";
    let (client, connection) = tokio_postgres::connect(connection_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let statement = client.prepare_typed(&query_string, &[]).await?;
    let rows = client.query(&statement, &[]).await?;

    Ok(rows)
}

pub fn get_connection() -> Client {
    Client::connect("postgres://postgres@localhost:5432/beer_tap_list", PgNoTls)
        .expect("pg connection is very virus")
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

// #[tokio::main]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:10001")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            // BEER
            .route("/beers", web::get().to(responders::get_beer_list))
            // .route("/beers/{id}", web::get().to(responders::get_beer_by_id))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
