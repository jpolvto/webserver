use actix_web::{HttpRequest, HttpResponse, Responder, web};
use bson::{doc};
use futures_util::StreamExt;
use crate::models;
use crate::models::{User, user_from_document};

pub async fn all_users(data: web::Data<models::AppState>) -> impl Responder {

    let mut cursor = data.data.find(doc! {}, None).await.unwrap();
    let mut results: Vec<User> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let optional_user = user_from_document(document);
                if let Some(user) = optional_user {
                    results.push(user);
                }
            }
            Err(_) => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::Ok().json(results)
}

pub async fn get_user_by_id(req: HttpRequest, data: web::Data<models::AppState>) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let result = data.data.find_one(doc! { "id":  id }, None).await.unwrap();
    let mut results: Vec<User> = Vec::new();

    if let Some(doc) = result {
        let optional_user = user_from_document(doc);
        if let Some(user) = optional_user {
            results.push(user);
        }
    }

    HttpResponse::Ok().json(results)
}

pub async fn post_user(info: web::Json<models::User>, data: web::Data<models::AppState>) -> impl Responder {
    let result = data.data.insert_one(doc! { "id": &info.id, "email": &info.email }, None).await.unwrap();
    HttpResponse::Ok().json(doc! { "_id": result.inserted_id, "id": &info.id, "email": &info.email })
}

pub async fn delete_user_by_id(req: HttpRequest, data: web::Data<models::AppState>) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let result = data.data.find_one_and_delete(doc! { "id":  id }, None).await.unwrap();


    let mut results: Vec<User> = Vec::new();

    if let Some(doc) = result {
        let optional_user = user_from_document(doc);
        if let Some(user) = optional_user {
            results.push(user);
        }
    }

    HttpResponse::Ok().json(results)
}