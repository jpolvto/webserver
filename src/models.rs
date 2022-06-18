use std::string::String;
use mongodb::{Collection};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct AppState {
    pub col: Collection<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>
}
