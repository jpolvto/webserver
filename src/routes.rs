use actix_web::{HttpRequest, HttpResponse, Responder, web};
use bson::{Bson, doc};
use futures_util::StreamExt;
use crate::models;
use crate::models::{User, user_from_document};

const MAX_SIZE: usize = 262_144; // max payload size is 256k

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

pub async fn post_users(mut payload: web::Payload, data: web::Data<models::AppState>) -> impl Responder {

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk_result) => {
                if (body.len() + chunk_result.len()) > MAX_SIZE { // limit max size of in-memory payload
                    return HttpResponse::InternalServerError().finish()
                }
                body.extend_from_slice(&chunk_result);}
            Err(_) => {
                return HttpResponse::BadRequest().finish()
            }
        };

    }

    let input;

    match String::from_utf8(body.to_vec()) {
        Ok(value) => {
            input = value;
        }
        Err(_) => {
            return HttpResponse::BadRequest().finish()
        }
    }

    let all_users: Vec<User>;

    match serde_json::from_str(&input) {
        Ok(converted_input) => {
            all_users = converted_input;
        }
        Err(_) => {
            return HttpResponse::BadRequest().finish()
        }
    }

    let mut docs = Vec::new();

    for user in all_users {
        docs.push(doc! { "id": user.id, "email": user.email })
    }

    match data.data.insert_many(docs, None).await {
        Ok(result) => {
            let inserts: Vec<Bson> = result.inserted_ids.into_values().collect();
            HttpResponse::Ok().json(doc! { "Entries inserted:": inserts }) }
        Err(_) => { HttpResponse::NotFound().finish() }
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

    return match data.data.delete_many(doc! { "id":  id }, None).await {
        Ok(result) => { HttpResponse::Ok().json(doc! { "Number of entries deleted:": result.deleted_count.to_string() }) }
        Err(_) => { HttpResponse::NotFound().finish() }
    };

}