mod models;
use actix_web::{web, App, HttpServer, Responder};

pub async fn get_users() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Start http server
    HttpServer::new(move || {
    App::new()
        .route("/users", web::get().to(get_users))
        .route("/users/{id}", web::get().to(get_user_by_id))
        .route("/users", web::get().to(add_user))
        .route("/users/{id}", web::get().to(delete_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
