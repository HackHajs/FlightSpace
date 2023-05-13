use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use regex::Regex;

const X_CENTER: u32 = 1920 / 2;
const Y_CENTER: u32 = 1080 / 2;

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

#[get("/loose_health/{player_name}")]
async fn loose_health(players: web::Data<Mutex<HashMap<String, Player>>>, player_name: web::Path<String>) -> impl Responder {
    let player_name = format!("{}", player_name);
    let mut guard = players.try_lock().unwrap();
    guard.entry(String::from(player_name)).and_modify(|player| player.health -= 1);
    std::mem::drop(guard);

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json("OK")
}

#[get("/pos/{data}")]
async fn pos(players: web::Data<Mutex<HashMap<String, Player>>>, data: web::Path<String>) -> impl Responder {
    let re = Regex::new(r"([^\/]+):([^\/]+):([^\/]+)").unwrap();
    let cap = re.captures_iter(&data).next().unwrap();
    let player_name = cap[1].to_string();
    let x_pos: u32 = cap[2].parse().unwrap();
    let y_pos: u32 = cap[3].parse().unwrap();
    let mut guard = players.try_lock().unwrap();
    guard.entry(String::from(player_name)).and_modify(|player| { 
        player.x_pos = x_pos;
        player.y_pos = y_pos;
    });
    std::mem::drop(guard);

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json("OK")
}


#[get("/join/{player_name}")]
async fn join(players: web::Data<Mutex<HashMap<String, Player>>>, player_name: web::Path<String>) -> impl Responder {
    let player_name = format!("{}", player_name);
    let mut guard = players.try_lock().unwrap();
    guard.insert(player_name.clone(), Player {
        name: player_name,
        health: 3,
        x_pos: X_CENTER,
        y_pos: Y_CENTER,
    });
    std::mem::drop(guard);

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json("OK")
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
            .service(loose_health)
            .service(playercount)
            .service(pos)
            .service(join)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
