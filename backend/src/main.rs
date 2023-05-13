use std::{
    collections::hash_map::HashMap,
};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod players;
use players::{parse_players, mod_players, Player, Message, Field};

#[get("/players")]
async fn get_players() -> impl Responder {
    HttpResponse::Ok().json(Message{ players: parse_players() })
}

#[get("/update/{player}/{field}/{value}")]
async fn update_player(player: web::Path<String>, field: web::Path<String>, value: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(todo!())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| App::new().service(get_players))
        .bind("127.0.0.1:8080")?
        .run()
    .await
}
