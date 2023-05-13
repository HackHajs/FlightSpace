use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Player {
    pub name: String,
    pub x_pos: u32,
    pub y_pos: u32,
    pub health: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    players: Mutex<HashMap<String, Player>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Playercount(usize);

#[get("/players")]
async fn get_players(players: web::Data<Mutex<HashMap<String, Player>>>) -> impl Responder {
    // This is a bit jank, but the clock is ticking
    let mut de_datafied_players: HashMap<String, Player> = HashMap::new();
    for (k, v) in players.lock().unwrap().iter() {
        de_datafied_players.insert(k.to_string(), v.clone());
    }

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(de_datafied_players)
}

#[get("/playercount")]
async fn playercount(players: web::Data<Mutex<HashMap<String, Player>>>) -> impl Responder {
    println!("{}", players.lock().unwrap().len());
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(Playercount(players.lock().unwrap().len()))
}

#[get("/update/{test}")]
async fn update(players: web::Data<Mutex<HashMap<String, Player>>>, test: web::Path<String>) -> impl Responder {
    let re = Regex::new(r"([^\/]+):([^\/]+):([^\/]+)").unwrap();
    let mut player = "";
    let mut field = "";
    let mut value = 0;

    let cap = re.captures_iter(&test).next().unwrap(); 
    player = &cap[1];
    field = &cap[2];
    value = cap[3].parse().unwrap();

    if field == "health" {
        players.lock().unwrap().entry(String::from(player)).and_modify(|player| player.health += value);
    } else if field == "xpos" {
        todo!();
    } else if field == "ypos" {
        todo!();
    }

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(format!("{}, {}, {}", player, field, value))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let players: Mutex<HashMap<String, Player>> = Mutex::from(HashMap::new());
        players.lock().unwrap().insert(
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
            .service(update)
            .service(playercount)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
