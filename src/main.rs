use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use bson::{ doc, Document};
use futures_util::stream::StreamExt;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use serde::{ Serialize, Deserialize};

pub struct AppState {
    pub data: Collection<Document>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    email: String
}


pub async fn all_users(data: web::Data<AppState>) -> impl Responder {

    let mut cursor = data.data.find(doc! {}, None).await.unwrap();
    let mut results: Vec<Document> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            Err(_) => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::Ok().json(results)
}

pub async fn get_user_by_id(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let result = data.data.find_one(doc! { "id":  id }, None).await.unwrap();
    let mut results: Vec<Document> = Vec::new();

    if let Some(doc) = result {
        results.push(doc);
    }

    HttpResponse::Ok().json(results)
}

pub async fn delete_user_by_id(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let result = data.data.find_one_and_delete(doc! { "id":  id }, None).await.unwrap();
    let mut results: Vec<Document> = Vec::new();

    if let Some(doc) = result {
        results.push(doc);
    }

    HttpResponse::Ok().json(results)
}

pub async fn post_user(info: web::Json<User>, data: web::Data<AppState>) -> impl Responder {

    let result = data.data.insert_one(doc! { "id": &info.id, "email": &info.email }, None).await.unwrap();
    HttpResponse::Ok().json(result.inserted_id)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let options = ClientOptions::parse("mongodb://127.0.0.1:27017").await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database("actix-web");

    HttpServer::new(move || {
        App::new()
            .app_data(db.collection::<Document>("users"))
            .route("/users", web::get().to(all_users))
            .route("/users", web::post().to(post_user))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users/{id}", web::delete().to(delete_user_by_id))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}