use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Ptt {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct PttMessagesCount {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct PttMessages {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    println!("Hello, world!");
}
