use std::collections::HashMap;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};

pub enum Field {
    Xpos(u32),
    Ypos(u32),
    Health(u8),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub x_pos: u32,
    pub y_pos: u32,
    pub health: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub players: HashMap<String, Player>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Playercount(usize);

#[get("/players")]
async fn get_players(players: web::Data<HashMap<String, Player>>) -> impl Responder {
    // This is a bit jank, but the clock is ticking
    let mut de_datafied_players: HashMap<String, Player> = HashMap::new();
    for (k, v) in players.iter() {
        de_datafied_players.insert(k.to_string(), v.clone());
    }

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(de_datafied_players)
}

#[get("/playercount")]
async fn playercount(players: web::Data<HashMap<String, Player>>) -> impl Responder {
    println!("{}", players.len());
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(Playercount(players.len()))
}

#[get("/update/{player}/{field}/{value}")]
async fn update_player(
    players: web::Data<HashMap<String, Player>>,
    player_name: web::Path<String>,
    field: web::Path<String>,
    value: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(todo!())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let mut players: HashMap<String, Player> = HashMap::new();
        players.insert(
            "Player1".to_string(),
            Player {
                name: "Player1".to_owned(),
                health: 3,
                x_pos: 100,
                y_pos: 100,
            },
        );
        App::new()
            .app_data(web::Data::new(players))
            .service(get_players)
            .service(update_player)
            .service(playercount)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
