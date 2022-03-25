extern crate alloc;

mod routes;
mod models;

use std::env;
use actix_web::{App, HttpServer, web};
use bson::{Document};
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use dotenv;
use crate::models::AppState;
use crate::routes::{all_users, delete_user_by_id, get_users_by_id, post_users};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let host_name = format!("mongodb+srv://{}:{}@cluster0.17s4f.mongodb.net/actix-webserver?retryWrites=true&w=majority", env::var("USER").unwrap(), env::var("PASSWORD").unwrap());
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
            .route("/users", web::post().to(post_users))
            .route("/users/{id}", web::get().to(get_users_by_id))
            .route("/users/{id}", web::delete().to(delete_user_by_id))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}