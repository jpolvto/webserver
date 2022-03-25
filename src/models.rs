use bson::{Document};
use std::string::String;
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

pub fn user_from_document(document: bson::Document) -> Option<User> {

    let _id = document.get("id");
    let _email = document.get("email");

    return match (_id, _email) {
        (Some(_id), Some(_email)) => {
            let a = _id.as_i32();
            let b = _email.as_str();
            return match (a, b) {
                (Some(a), Some(b)) => return Some(User{ id: a, email: b.to_string() }),
                _ => None,
            }
        },
        _ => None,
    }
}