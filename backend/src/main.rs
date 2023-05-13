use std::{
    collections::hash_map::HashMap,
    fs::{ 
        read_to_string,
        write,
        File,
    },
    io::ErrorKind,
};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use regex::Regex;

enum Field {
    Xpos(u32),
    Ypos(u32),
    Health(u8),
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: String,
    x_pos: u32,
    y_pos: u32,
    health: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    players: HashMap<String, Player>,
}

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

fn parse_players() -> HashMap<String, Player> {
    // Get player save file path
    let mut player_file_path = home::home_dir().unwrap();
    player_file_path.push("Demo");
    player_file_path.push("players");

    // Read or create the save file
    let unparsed_players = match read_to_string(&player_file_path) {
        Ok(file) => file, 
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                File::create(&player_file_path).expect("Failed to create config file");
                String::new()
            },
            _ => panic!("Failed to create save file!"),
        }
    };

    let mut players: HashMap<String, Player> = HashMap::new();
    
    let re = Regex::new(r"\(([^:]+):(\d+):(\d+):(\d+)\)").unwrap();
    for cap in re.captures_iter(&unparsed_players) {
        let player = Player {
            name: cap[1].to_string(),
            health: cap[2].parse().unwrap(),
            x_pos: cap[3].parse().unwrap(),
            y_pos: cap[4].parse().unwrap(),
        };
        players.insert(cap[1].to_string(), player);
    }

    players
}

fn write_players(field: Field, name: String) {
    match field {
        _ => todo!(),
    }
}
