extern crate alloc;
extern crate env_logger;
extern crate core;

mod routes;
mod models;
mod errors;

use std::env;
use actix_web::{App, error, HttpResponse, HttpServer, middleware, ResponseError, web};
use actix_web::middleware::Logger;
use actix_web::web::{QueryConfig};
use mongodb::{Client};
use mongodb::options::ClientOptions;
use dotenv;
use crate::errors::{ErrorResponse};
use crate::models::AppState;
use crate::routes::{delete_users, get_users, post_users, put_users};

#[actix_web::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> tokio::io::Result<()> {
    dotenv::dotenv().ok();

    let database_name = env::var("DATABASE_NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let user = env::var("USER").unwrap();

    // custom `Json` extractor configuration
    let json_cfg = web::JsonConfig::default()
        // limit request payload size
        .limit(4096)
        // only accept text/plain content type
        .content_type(|mime| mime == mime::TEXT_PLAIN)
        // use custom error handler
        .error_handler(|err, _req| {

            let status_code = err.status_code();
            let response = ErrorResponse::from(&err);
            error::InternalError::from_response(err, HttpResponse::build(status_code)
                .json(response)).into()

        });

    let query_cfg = QueryConfig::default()
        .error_handler(|err, _req| {

            let status_code = err.status_code();
            let response = ErrorResponse::from(&err);
            error::InternalError::from_response(err, HttpResponse::build(status_code)
                .json(response)).into()

        });


    let host_name = format!("mongodb+srv://{}:{}@cluster0.17s4f.mongodb.net/{}?retryWrites=true&w=majority",
                            &user,
                            &password,
                            &database_name,
    );

    let options = ClientOptions::parse(&host_name).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database(&database_name);

    let data = web::Data::new(AppState {
        db,
    });

    env_logger::init();

    HttpServer::new(move || {

        App::new()
            .app_data(data.clone())
            .app_data(json_cfg.clone())
            .app_data(query_cfg.clone())
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(post_users)
            .service(get_users)
            .service(delete_users)
            .service(put_users)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}