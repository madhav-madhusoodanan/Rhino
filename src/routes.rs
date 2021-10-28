use actix_web::{get, web, post, Responder, HttpResponse};
use crate::config::{ Request, Body, State };

/* router functions */
#[get("/")]
pub async fn yo() -> impl Responder {
    HttpResponse::Ok().body("Yea yea i am good :)")
}

#[post("/get")]
pub async fn get(state: web::Data<State>, body: web::Json<Request>) -> impl Responder {
    let data = state.data.lock().unwrap();
    match data.get(&body.key){
        Some(response) => HttpResponse::Ok().body(response),
        None => return HttpResponse::NoContent().finish()
    }
    
}

#[post("/add")]
pub async fn add(state: web::Data<State>, body: web::Json<Body>) -> impl Responder {
    let mut data = state.data.lock().unwrap();
    data.insert(body.key.to_string(), body.value.to_string());
    HttpResponse::Created().finish()
}
