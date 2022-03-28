use std::num::ParseIntError;
use bson::{Bson, doc, Document};
use futures_util::StreamExt;
use mongodb::Collection;
use crate::models;
use crate::models::{User};
use actix_web::{get, post, delete, web::Json, HttpResponse, http::{header::ContentType, StatusCode}, web, HttpRequest, error};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "not found")]
    NotFound,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(format!("{{ \"status_code\":{}, \"message\": \"{}\" }}", self.status_code().as_u16(), self.to_string()))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadClientData => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}


impl From<mongodb::error::Error> for AppError {
    fn from(_error: mongodb::error::Error) -> Self {
         return AppError::InternalError;
    }
}

impl From<ParseIntError> for AppError {
    fn from(_error: ParseIntError) -> Self {
        return AppError::BadClientData;
    }
}

#[get("/users")]
pub async fn all_users(data: web::Data<models::AppState>) -> Result<Json<Vec<User>>, AppError> {

    let user_collection: Collection<Document> = data.db.collection("users");
    let mut cursor= user_collection.find(doc! { }, None).await?;

    let mut results: Vec<User> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let optional_user = User::from_document(document);
                if let Some(user) = optional_user {
                    results.push(user);
                }
            }
            Err(err) => {
                drop(err);
            }
        }
    }

    if results.is_empty() {
        return Err(AppError::NotFound)
    }

    Ok(Json(results))

}

#[get("/users/{id}")]
pub async fn get_users_by_id(req: HttpRequest, data: web::Data<models::AppState>) -> Result<Json<Vec<User>>, AppError> {

    let id: i32;

    match req.match_info().get("id") {
        Some(id_cast) => {
            id = id_cast.parse::<i32>()?
        }
        None => {
            return Err(AppError::BadClientData)
        }
    };


    let user_collection: Collection<Document> = data.db.collection("users");
    let mut cursor= user_collection.find(doc! { "id": id }, None).await?;
    let mut results: Vec<User> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let optional_user = User::from_document(document);
                if let Some(user) = optional_user {
                    results.push(user);
                }
            }
            Err(err) => {
                drop(err);
            }
        }
    }

    if results.is_empty() {
        return Err(AppError::NotFound)
    }

    Ok(Json(results))
}

#[post("/users")]
pub async fn post_users(info: web::Json<Vec<User>>, data: web::Data<models::AppState>) -> Result<Json<Document>, AppError> {

    let mut docs: Vec<Document> = Vec::new();

    for user in info.0 {
        let doc = User::into(user);
        docs.push(doc)
    }

    let user_collection: Collection<Document> = data.db.collection("users");
    let result:Vec<Bson> =  user_collection.insert_many(docs, None).await?.inserted_ids.into_values().collect();

    Ok(Json(doc!{ "entries_inserted": result }))
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(req: HttpRequest, data: web::Data<models::AppState>) -> Result<Json<Document>, AppError> {

    let id: i32;

    match req.match_info().get("id") {
        Some(id_cast) => {
            id = id_cast.parse::<i32>()?
        }
        None => {
            return Err(AppError::BadClientData)
        }
    };

    let user_collection: Collection<Document> = data.db.collection("users");
    let result =  user_collection.delete_many(doc! { "id":  id }, None).await?.deleted_count.to_string();

    Ok(Json(doc!{ "number_of_entries_deleted:": result }))

}