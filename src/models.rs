use std::string::String;
use mongodb::{Database};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct AppState {
    pub db: Database,
}

#[derive(Deserialize)]
pub struct Info {
    pub id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>
}


