extern crate alloc;
extern crate env_logger;
extern crate core;

mod routes;
mod models;
mod errors;

use std::env;
use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::Logger;
use mongodb::{Client};
use mongodb::options::ClientOptions;
use dotenv;
use crate::models::AppState;
use crate::routes::{all_users, delete_user_by_id, get_users_by_id, post_users};

#[actix_web::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> tokio::io::Result<()> {
    dotenv::dotenv().ok();

    let host_name = format!("mongodb+srv://{}:{}@cluster0.17s4f.mongodb.net/actix-webserver?retryWrites=true&w=majority",
                            env::var("USER").expect("No user found"),
                            env::var("PASSWORD").expect("No password found"));
    let options = ClientOptions::parse(&host_name).await.expect("Error parsing client options");
    let client = Client::with_options(options).expect("Error creating client");
    let db = client.database("actix-webserver");

    let data = web::Data::new(AppState {
        db,
    });

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // <- register the created data
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(all_users)
            .service(post_users)
            .service(get_users_by_id)
            .service(delete_user_by_id)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}