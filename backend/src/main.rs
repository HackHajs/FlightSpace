use std::collections::hash_map::*;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    players: HashMap<String, (i32, i32)>,
}


#[get("/players")]
async fn get_players() -> impl Responder {
    HttpResponse::Ok().json(Message {
        players: HashMap::from([("Player1".to_owned(), (10, 10)), ("Player2".to_owned(), (10, 20))]),
    })
}


#[get("/update/{player}")]
async fn get_sussers() -> impl Responder {
    HttpResponse::Ok().json(Message {
        players: HashMap::from([("Player1".to_owned(), (10, 10))]),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_players).service(get_sussers))
        .bind("127.0.0.1:8080")?
        .run()
    .await
}
