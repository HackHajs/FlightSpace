use std::collections::hash_map::HashMap;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod players;
use players::{mod_players, parse_players, Field, Message, Player};

#[get("/players")]
async fn get_players(players: web::Data<HashMap<String, Player>>) -> impl Responder {
    // This is a bit jank, but the clock is ticking
    let mut de_datafied_players: HashMap<String, Player> = HashMap::new();
    for (k, v) in players.iter() {
        de_datafied_players.insert(k.to_string(), v.clone());
    }

    HttpResponse::Ok().json(Message {
        players: de_datafied_players
    })
}

#[get("/update/{player}/{field}/{value}")]
async fn update_player(
    players: web::Data<HashMap<String, Player>>,
    player_name: web::Path<String>,
    field: web::Path<String>,
    value: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().json(todo!())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(parse_players()))
            .service(get_players)
            .service(update_player)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
