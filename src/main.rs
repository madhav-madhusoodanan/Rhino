use actix_web::{get, web, post, App, HttpServer, Responder, HttpResponse};
use std::collections::HashMap;
use std::sync::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
enum RequestType {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Deserialize)]
struct Request {
    key: String,
}

#[derive(Deserialize)]
struct Body {
    key: String,
    value: String
}

/* The shared state */
struct State {
    data: Mutex<HashMap<String, String>>,
}

impl State {
    pub fn new() -> State {
        State { data: Mutex::new(HashMap::new())
            }
    }
}

/* router functions */
#[get("/")]
async fn yo() -> impl Responder {
    HttpResponse::Ok().body("Yea yea i am good :)")
}

#[post("/get")]
async fn get(state: web::Data<State>, body: web::Json<Request>) -> impl Responder {
    let data = state.data.lock().unwrap();
    let response = data.get(&body.key).unwrap();
    HttpResponse::Ok().body(response)
}

#[post("/add")]
async fn add(state: web::Data<State>, body: web::Json<Body>) -> impl Responder {
    let mut data = state.data.lock().unwrap();
    data.insert(body.key.to_string(), body.value.to_string());
    HttpResponse::Ok().finish()
}

/* Setup configurations */
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State::new());
    let factory = move || {
        App::new()
                .app_data(state.clone())
                .service(yo)
                .service(get)
                .service(add)
    };

    HttpServer::new(factory).bind("127.0.0.1:8080")?.run().await
}