use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: String,
    pos: (u32, u32),
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    players: Vec<Player>,
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Message {
        players: vec![Player {
            name: "Player1".to_owned(),
            pos: (10, 10),
        },
        Player {
            name: "Player2".to_owned(),
            pos: (20, 20)
        }],
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8080")?
        .run()
    .await
}
