use std::env;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use bson::{doc, Document};
use futures_util::stream::StreamExt;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use dotenv;
use serde::{ Serialize, Deserialize};

pub struct AppState {
    pub data: Collection<Document>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String
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

pub async fn post_user(info: web::Json<User>, data: web::Data<AppState>) -> impl Responder {

    let result = data.data.insert_one(doc! { "id": &info.id, "email": &info.email }, None).await.unwrap();
    HttpResponse::Ok().json(doc! { "_id": result.inserted_id, "id": &info.id, "email": &info.email })
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let host_name = format!("mongodb+srv://{}:{}@cluster0.17s4f.mongodb.net/actix-webserver?retryWrites=true&w=majority", env::var("user").unwrap(), env::var("password").unwrap());
    let options = ClientOptions::parse(&host_name).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database("actix-webserver");
    let user_collection: Collection<Document> = db.collection("users");

    let data = web::Data::new(AppState {
        data: user_collection,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // <- register the created data
            .route("/users", web::get().to(all_users))
            .route("/users", web::post().to(post_user))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users/{id}", web::delete().to(delete_user_by_id))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}