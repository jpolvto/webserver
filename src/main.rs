extern crate alloc;
extern crate env_logger;

mod routes;
mod models;

use std::env;
use actix_web::{App, HttpServer, middleware, web};
use mongodb::{Client};
use mongodb::options::ClientOptions;
use dotenv;
use crate::models::AppState;
use crate::routes::{all_users, delete_user_by_id, get_users_by_id, post_users};

#[actix_web::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> tokio::io::Result<()> {
    dotenv::dotenv().ok();

    let host_name = format!("mongodb+srv://{}:{}@cluster0.17s4f.mongodb.net/actix-webserver?retryWrites=true&w=majority", env::var("USER").unwrap(), env::var("PASSWORD").unwrap());
    let options = ClientOptions::parse(&host_name).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database("actix-webserver");

    let data = web::Data::new(AppState {
        db,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // <- register the created data
            .wrap(middleware::Compress::default())
            .service(all_users)
            .service(post_users)
            .service(get_users_by_id)
            .service(delete_user_by_id)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}