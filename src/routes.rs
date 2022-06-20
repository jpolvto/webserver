use bson::{doc, to_document};
use futures_util::{TryStreamExt};
use crate::models;
use crate::models::User;
use actix_web::{post, get, put, delete, web::Json, web};
use mongodb::results::{DeleteResult, InsertManyResult, UpdateResult};
use crate::errors::AppError;

#[get("/users")]
pub async fn get_users(info: web::Query<User>, data: web::Data<models::AppState>) -> Result<Json<Vec<User>>, AppError> {
    
    let results = data.col.find(to_document(&info.into_inner())?, None).await?.try_collect().await.unwrap_or_else(|_| vec![]);

    if results.is_empty() {
        return Err(AppError::NotFound);
    }

    Ok(Json(results))
}

#[delete("/users")]
pub async fn delete_users(info: web::Query<User>, data: web::Data<models::AppState>) -> Result<Json<DeleteResult>, AppError> {

    let results = data.col.delete_many(to_document(&info.into_inner())?, None).await?;

    if results.deleted_count == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(results))
}

#[put("/users")]
pub async fn put_users(input: web::Json<User>, info: web::Query<User>, data: web::Data<models::AppState>) -> Result<Json<UpdateResult>, AppError> {

    let results = data.col.update_many(to_document(&info.into_inner())?, doc!{ "$set": bson::to_bson(&input.into_inner())? }, None).await?;

    if results.matched_count == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(results))
}

#[post("/users")]
pub async fn post_users(input: web::Json<Vec<User>>, data: web::Data<models::AppState>) -> Result<Json<InsertManyResult>, AppError> {

    let results = data.col.insert_many(input.into_inner(), None).await?;
    
    if results.inserted_ids.is_empty() {
        return Err(AppError::NotFound);
    }

    Ok(Json(results))
}