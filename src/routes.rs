use bson::{doc, Document};
use futures_util::StreamExt;
use mongodb::{Collection, Cursor};
use crate::models;
use crate::models::User;
use crate::models::Info;
use actix_web::{post, get, put, delete, web::Json, web};
use mongodb::results::{DeleteResult, InsertManyResult, UpdateResult};
use crate::errors::AppError;

pub async fn get_users_from_cursor (mut cursor: Cursor<Document>) -> Result<Vec<User>, AppError> {
    let mut results: Vec<User> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                let deserialized_user: User = bson::from_document(doc)?;
                results.push(deserialized_user)
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
pub async fn delete_users_by_id(info: web::Path<Info>, data: web::Data<models::AppState>) -> Result<Json<DeleteResult>, AppError> {

    let user_collection: Collection<Document> = data.db.collection("users");
    let result =  user_collection.delete_many(doc! { "id":  info.id }, None).await?;

    Ok(Json(result))

}

#[put("/users/{id}")]
pub async fn put_users_by_id(input: web::Json<User>, info: web::Path<Info>, data: web::Data<models::AppState>) -> Result<Json<UpdateResult>, AppError> {

    let user = input.into_inner();
    let serialized_person = bson::to_bson(&user)?;
    let user_collection: Collection<Document> = data.db.collection("users");
    let result =  user_collection.update_many(doc! { "id":  info.id }, doc!{ "$set": serialized_person }, None).await?;

    Ok(Json(result))

}

#[post("/users")]
pub async fn post_users(input: web::Json<Vec<User>>, data: web::Data<models::AppState>) -> Result<Json<InsertManyResult>, AppError> {

    let mut docs: Vec<Document> = Vec::new();

    for user in input.0 {
        let serialized_user = bson::to_bson(&user)?;  // Serialize

        if let bson::Bson::Document(document) = serialized_user {
            docs.push(document)
        }
    }

    let user_collection: Collection<Document> = data.db.collection("users");
    let result =  user_collection.insert_many(docs, None).await?;

    Ok(Json(result))
}

