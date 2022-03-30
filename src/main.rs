extern crate alloc;
extern crate env_logger;
extern crate core;

mod routes;
mod models;
mod errors;

use std::env;
use actix_web::{App, error, HttpResponse, HttpServer, middleware, ResponseError, web};
use actix_web::middleware::Logger;
use actix_web::web::{PathConfig};
use mongodb::{Client};
use mongodb::options::ClientOptions;
use dotenv;
use crate::errors::{ErrorResponse};
use crate::models::AppState;
use crate::routes::{all_users, delete_user_by_id, get_users_by_id, post_users};

#[actix_web::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> tokio::io::Result<()> {
    dotenv::dotenv().ok();

    // custom `Json` extractor configuration
    let json_cfg = web::JsonConfig::default()
        // limit request payload size
        .limit(4096)
        // only accept text/plain content type
        .content_type(|mime| mime == mime::TEXT_PLAIN)
        // use custom error handler
        .error_handler(|err, req| {
            let error_code = err.status_code();
            let error_response = ErrorResponse {
                code: error_code.as_u16(),
                message: err.to_string(),
            };
            error::InternalError::from_response(err, HttpResponse::build(error_code).json(error_response)).into()
        });

    let path_cfg = PathConfig::default()
        .error_handler(|err, req| {
            let error_code = err.status_code();

            let error_response = ErrorResponse {
                code: error_code.as_u16(),
                message: err.to_string(),
            };
            error::InternalError::from_response(err, HttpResponse::build(error_code).json(error_response)).into()
        });

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
            .app_data(json_cfg.clone())
            .app_data(path_cfg.clone())
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