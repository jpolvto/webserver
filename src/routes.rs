use actix_web::{HttpRequest, HttpResponse, Responder, web};
use bson::{doc};
use futures_util::StreamExt;
use crate::models;
use crate::models::{User, user_from_document};

pub async fn all_users(data: web::Data<models::AppState>) -> impl Responder {

    let mut cursor= match data.data.find(doc! {}, None).await {
        Ok(result) => result,
        Err(_) => {return HttpResponse::NotFound().finish();}
    };

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

    let result = match data.data.find_one(doc! { "id":  id }, None).await {
        Ok(result) => result,
        Err(_) => {return HttpResponse::NotFound().finish()}
    };

    match result {
        Some(doc) => {
            let mut results: Vec<User> = Vec::new();

            let optional_user = user_from_document(doc);
            if let Some(user) = optional_user {
                results.push(user);
            }
            HttpResponse::Ok().json(results)
        }
        None => {
            return HttpResponse::NotFound().finish()
        }
    }
}

pub async fn post_user(info: web::Json<models::User>, data: web::Data<models::AppState>) -> impl Responder {

    match data.data.insert_one(doc! { "id": &info.id, "email": &info.email }, None).await {
        Ok(_) => {
            HttpResponse::Ok().json(doc! { "id": &info.id, "email": &info.email })
        }
        Err(_) => {
            return HttpResponse::InternalServerError().finish()
        }
    }
}

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

    let result = match data.data.find_one_and_delete(doc! { "id":  id }, None).await {
        Ok(result) => result,
        Err(_) => {return HttpResponse::NotFound().finish()}
    };

    match result {
        Some(doc) => {
            let mut results: Vec<User> = Vec::new();

            let optional_user = user_from_document(doc);
            if let Some(user) = optional_user {
                results.push(user);
            }
            HttpResponse::Ok().json(results)
        }
        None => {
            return HttpResponse::NotFound().finish()
        }
    }
}