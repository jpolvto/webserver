mod routers;
mod models;

use actix_web::{App, HttpServer, web};
use bson::{ Document};
use mongodb::{Client};
use mongodb::options::ClientOptions;

use crate::routers::{all_users, delete_user_by_id, get_user_by_id, post_user};

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