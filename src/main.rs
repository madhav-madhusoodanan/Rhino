/* Crate importing */
use actix_web::{ web, App, HttpServer };

/* Local modules */
mod config;
mod routes;
use crate::config::{ State };
use crate::routes::{ yo, get, add };

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