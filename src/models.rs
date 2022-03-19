// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub attribute: String,
    pub value: String
}