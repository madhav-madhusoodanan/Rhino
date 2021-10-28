use std::collections::HashMap;
use std::sync::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum RequestType {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Deserialize)]
pub struct Request {
    pub key: String,
}

#[derive(Deserialize)]
pub struct Body {
    pub key: String,
    pub value: String
}

/* The shared state */
pub struct State {
    pub data: Mutex<HashMap<String, String>>,
}

impl State {
    pub fn new() -> State {
        State { data: Mutex::new(HashMap::new())
            }
    }
}
