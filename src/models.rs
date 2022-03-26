use bson::{doc, Document};
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
impl TryFrom<&bson::Document> for User {
    type Error = ();

    fn try_from(value: &Document) -> Result<Self, ()> {

        let _id = value.get("id");
        let _email = value.get("email");

        match (_id, _email) {
            (Some(_id), Some(_email)) => {
                let a = _id.as_i32();
                let b = _email.as_str();
                return match (a, b) {
                    (Some(a), Some(b)) => return Ok(User{ id: a, email: b.to_string() }),
                    _ => Err(()),
                }
            },
            _ => Err(()),
        }

    }
}

impl Into<bson::Document> for User {
    fn into(self) -> Document {
        doc! {
            "id": self.id,
            "email": self.email,
        }
    }
}
