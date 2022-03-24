use bson::Document;
use mongodb::Collection;
use serde::{Serialize, Deserialize};

pub struct AppState {
    pub data: Collection<Document>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String
}