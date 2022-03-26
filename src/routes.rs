use actix_web::{HttpRequest, HttpResponse, Responder, web, get, delete, post};
use bson::{Bson, doc};
use futures_util::StreamExt;
use crate::models;
use crate::models::{User, document_to_user, user_to_document};

#[get("/users")]
pub async fn all_users(data: web::Data<models::AppState>) -> impl Responder {

    let mut cursor= match data.data.find(doc! {}, None).await {
        Ok(result) => result,
        _ => {return HttpResponse::NotFound().finish();}
    };

    let mut results: Vec<User> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let optional_user = document_to_user(document);
                if let Some(user) = optional_user {
                    results.push(user);
                }
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::Ok().json(results)
}

#[get("/users/{id}")]
pub async fn get_users_by_id(req: HttpRequest, data: web::Data<models::AppState>) -> impl Responder {

    let id: i32 = match req.match_info().get("id") {
        Some(id_cast) => {
            let id_parse = id_cast.parse::<i32>();
            match id_parse {
                Ok(_id) => _id,
                _ => {
                    return HttpResponse::BadRequest().finish()
                }
            }
        }
        None => {
            return HttpResponse::BadRequest().finish()
        }
    };


    let mut cursor= match data.data.find(doc! { "id":  id }, None).await {
        Ok(result) => result,
        _ => {return HttpResponse::NotFound().finish();}
    };

    let mut results: Vec<User> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let optional_user = document_to_user(document);
                if let Some(user) = optional_user {
                    results.push(user);
                }
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::Ok().json(results)
}

#[post("/users")]
pub async fn post_users(info: web::Json<Vec<User>>, data: web::Data<models::AppState>) -> impl Responder {

    let mut docs = Vec::new();

    for user in info.0 {
        docs.push(user_to_document(user))
    }

    match data.data.insert_many(docs, None).await {
        Ok(result) => {
            let inserts: Vec<Bson> = result.inserted_ids.into_values().collect();
            HttpResponse::Ok().json(doc! { "Entries inserted:": inserts }) }
        _ => { HttpResponse::NotFound().finish() }
    }
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(req: HttpRequest, data: web::Data<models::AppState>) -> impl Responder {

    let id: i32 = match req.match_info().get("id") {
        Some(id_cast) => {
            let id_parse = id_cast.parse::<i32>();
            match id_parse {
                Ok(_id) => _id,
                _ => {
                    return HttpResponse::BadRequest().finish()
                }
            }
        }
        None => {
            return HttpResponse::BadRequest().finish()
        }
    };

    return match data.data.delete_many(doc! { "id":  id }, None).await {
        Ok(result) => { HttpResponse::Ok().json(doc! { "Number of entries deleted:": result.deleted_count.to_string() }) }
        _ => { HttpResponse::NotFound().finish() }
    };
}