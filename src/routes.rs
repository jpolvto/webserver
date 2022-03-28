use bson::{Bson, doc, Document};
use futures_util::StreamExt;
use mongodb::Collection;
use crate::models;
use crate::models::User;
use actix_web::{get, post, delete, web::Json, web, HttpRequest};
use crate::errors::AppError;


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
    let result: i64 =  user_collection.delete_many(doc! { "id":  id }, None).await?.deleted_count as i64;

    Ok(Json(doc!{ "number_of_entries_deleted:": result }))

}