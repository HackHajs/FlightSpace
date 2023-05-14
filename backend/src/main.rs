use std::collections::HashMap;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use regex::Regex;
use rand::Rng;

mod players;
use players::{save_players, parse_players, player_side, Player, Message, X_CENTER, Y_CENTER};

mod questions;
use questions::*;

/// Reads the save file and sends a json object with all the players to the caller.
/// # How to use:
/// The request structure is the following:
/// `http://ip_address/players`
///
/// The json file that's replied back is structured like the following:
/// ```json
/// players:
///     <PLAYER1_NAME>:
///         name: "<PLAYER_NAME>"
///         xpos: <XPOS>
///         ypos: <YPOS>
///         health: <HEALTH>
///     <PLAYER2_NAME>:
///         name: "<PLAYER2_NAME>"
///         ...
///     <PLAYER3_NAME>:
///     ...
/// ```
#[get("/players")]
async fn get_players() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(Message {
            players: parse_players(),
        })
}

/// Reads the save file and updates the requested player stats.
/// # How to use
/// The request structure is the following: 
/// `http://ip_address/update/<NAME>:<STAT>:<VALUE>`
/// - `<NAME>` is replaced with the username of the player you're trying yo update 
/// - `<STAT>` is replaced with the stat that you want to update, which can be:
///     - `health`, which is deprecated in favor of `/judge`
///     - `xpos`, which means you want to update the position of the player in the X coordenate
///     - `ypos`, which means you want to update the position of the player in the Y coordinate
/// The reply will simply be a json object containing "OK" if the operation succeeded.
#[get("/update/{data}")]
async fn update_player(data: web::Path<String>) -> impl Responder {
    let re = Regex::new(r"([^\/]+):([^\/]+):([^\/]+)").unwrap();
    let cap = re.captures_iter(&data).next().unwrap();
    let player = cap[1].to_string();
    let field = &cap[2];
    let value: i32 = cap[3].parse().unwrap();

    let mut players = parse_players();

    match field {
        "health" => {
            players
                .entry(player)
                .and_modify(|player| player.health += value);
        }
        "xpos" => {
            players
                .entry(player)
                .and_modify(|players| players.x_pos = value);
        }
        "ypos" => {
            players
                .entry(player)
                .and_modify(|players| players.y_pos = value);
        }
        _ => (),
    };
    save_players(&players);

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json("OK")
}

/// Returns a json object containing a single number of players in the game session
/// # How to use
/// The request structure is the following:
/// `http://ip_address/count`
#[get("/count")]
async fn player_count() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(parse_players().len())
}

/// Creates a new player with the provided username. The new player will have the following
/// defaults: Health = 3, X and Y positions = Center of HTML5 canvas.
/// # How to use
///
/// The request structure is the following:
/// `http://ip_address/join/<USERNAME>`
/// - <USERNAME> is the username of the player you wish to create
#[get("/join/{name}")]
async fn create_player(name: web::Path<String>) -> impl Responder {
    let mut players = parse_players();
    players.insert(
        name.to_string(),
        Player {
            name: name.to_string(),
            health: 3,
            x_pos: X_CENTER,
            y_pos: Y_CENTER,
        },
    );

    save_players(&players);

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json("OK")
}

/// Replies with a random question from the set
/// # How to use
/// The request structure is the following
/// `http://ip_address/question`
///
/// The returned json object has the following structure:
/// ```json
/// question: "<QUESTION_TEXT>"
/// a: "<POSSIBLE_ANSWER_1>"
/// b: "<POSSIBLE_ANSWER_2>"
/// correct: "<CORRECT>"
/// number: <QUESTION_ID>
/// ```
/// - `<CORRECT>` is either `a` or `b`, representing one of the possible answers
/// - `<QUESTION_ID>` is supplied so players can later be judged on the correct question
#[get("/question")]
async fn send_question() -> impl Responder {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(1..=10);
    let question = get_question(random_number);
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(question)
}

/// Judges if players picked the right question. If it's incorrect, 1HP is substracted from their
/// health
/// # How to use
/// The request structure is the following:
/// `http://ip_address/judge/<QUESTION_ID>`
/// where the `<QUESTION_ID>` is that which was supplied by the `/question` call
/// It will reply with a json object containing "OK" if the operation succeeded.
#[get("/judge/{question}")]
async fn judge(question: web::Path<String>) -> impl Responder {
    let mut players = parse_players();
    let correct = get_question(question.parse().unwrap()).correct;

    for (_, player) in players.iter_mut() {
        if player_side(player.x_pos) != correct {
            player.health -= 1;
        }
    }
    save_players(&players);

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json("OK")
}

/// Deletes all the players and their data.
/// # How to use:
/// The request structure is the following:
/// `http://ip_address/clear`
#[get("/clear")]
async fn clear() -> impl Responder {
    let empty: HashMap<String, Player> = HashMap::new();
    save_players(&empty);

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json("OK")
}

/// Returns a specific question when provided with the question ID.
/// # How to use:
/// The request structure is the following:
/// `http://ip_address/questionid/<QUESTION_ID>`
/// where `<QUESTION_ID>` is the id of the question you want to receive
///
/// The returned json object has the following structure:
/// ```json
/// question: "<QUESTION_TEXT>"
/// a: "<POSSIBLE_ANSWER_1>"
/// b: "<POSSIBLE_ANSWER_2>"
/// correct: "<CORRECT>"
/// number: <QUESTION_ID>
/// ```
/// - `<CORRECT>` is either `a` or `b`, representing one of the possible answers
/// - `<QUESTION_ID>` is supplied so players can later be judged on the correct question
#[get("/questionid/{id}")]
async fn questionid(id: web::Path<String>) -> impl Responder {
    let question = get_question(id.parse().unwrap());
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(question)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enables logs. Only necessary for debug builds.
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(get_players)
            .service(update_player)
            .service(player_count)
            .service(create_player)
            .service(send_question)
            .service(judge)
            .service(clear)
            .service(questionid)
    })
    .bind("127.0.0.1:8080")? // Bound to localhost for the demo. Can use another IP for an on-board
    .run()                   // entretainment system.
    .await
}
