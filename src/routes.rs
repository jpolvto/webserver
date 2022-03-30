use bson::{doc, Document};
use futures_util::StreamExt;
use mongodb::{Collection, Cursor};
use crate::models;
use crate::models::User;
use crate::models::Info;
use actix_web::{post, get, delete, web::Json, web};
use mongodb::results::{DeleteResult, InsertManyResult};
use crate::errors::AppError;

pub async fn get_users_from_cursor (mut cursor: Cursor<Document>) -> Result<Vec<User>, AppError> {
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
        return Err(AppError::NotFound);
    }

    Ok(results)

}



#[get("/users")]
pub async fn all_users(data: web::Data<models::AppState>) -> Result<Json<Vec<User>>, AppError> {

    let user_collection: Collection<Document> = data.db.collection("users");
    let cursor= user_collection.find(doc! { }, None).await?;
    let results = get_users_from_cursor(cursor).await?;

    Ok(Json(results))

}

#[get("/users/{id}")]
pub async fn get_users_by_id(info: web::Path<Info>, data: web::Data<models::AppState>) -> Result<Json<Vec<User>>, AppError> {

    let user_collection: Collection<Document> = data.db.collection("users");
    let cursor= user_collection.find(doc! { "id": info.id }, None).await?;
    let results = get_users_from_cursor(cursor).await?;

    Ok(Json(results))
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(info: web::Path<Info>, data: web::Data<models::AppState>) -> Result<Json<DeleteResult>, AppError> {

    let user_collection: Collection<Document> = data.db.collection("users");
    let result =  user_collection.delete_many(doc! { "id":  info.id }, None).await?;

    Ok(Json(result))

}


#[post("/users")]
pub async fn post_users(info: web::Json<Vec<User>>, data: web::Data<models::AppState>) -> Result<Json<InsertManyResult>, AppError> {

    let mut docs: Vec<Document> = Vec::new();

    for user in info.0 {
        let doc = User::into(user);
        docs.push(doc)
    }

    let user_collection: Collection<Document> = data.db.collection("users");
    let result =  user_collection.insert_many(docs, None).await?;

    Ok(Json(result))
}
