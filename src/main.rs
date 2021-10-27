use actix_web::{get, /* web, */ post, App, HttpServer, Responder, HttpResponse};
use std::collections::HashMap;
use std::sync::Mutex;

const MAX_MEMORY: usize = 1024 * 1024 * 512;

struct State {
    data: Mutex<HashMap<String, String>>,
    memory: Mutex<usize>,
}

impl State {
    pub fn new() -> State {
        State { data: Mutex::new(HashMap::new()), 
                memory: Mutex::new(0), 
            }
    }

    pub fn add(&mut self, key: String, value: String) -> bool {
        let mut memory = self.memory.lock().unwrap();
        let should_add = *memory + key.capacity() + value.capacity() < MAX_MEMORY;
        if should_add {
            let mut data = self.data.lock().unwrap();
            *memory += key.capacity() + value.capacity();
            data.insert(key, value);
        }

        return should_add;
    }

    pub fn get(&self, key: &String) -> Option<String> {
        let data = self.data.lock().unwrap();
        match data.get(key) {
            Some(book) => Some(book.to_string()),
            None => None
        }
    }
}

/* router functions */
#[get("/")]
async fn get(/* data: web::Data<State> */) -> impl Responder {
    HttpResponse::Ok().body("")
}

#[post("/")]
async fn post(/* data: web::Data<State> */) -> impl Responder {
    HttpResponse::Ok().body("")
}

/* Setup configurations */
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let factory = move || {
        App::new()
                .app_data(State::new())
                .service(get)
    };

    HttpServer::new(factory).bind("127.0.0.1:8080")?.run().await
}