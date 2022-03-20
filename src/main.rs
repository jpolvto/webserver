mod models;
use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::models::User;

pub async fn get_users() -> impl Responder {
    let data :Vec<User> = get_users_from_json();
    let serialized = serde_json::to_string(&data).unwrap();
    format!("{}", serialized)
}

pub async fn get_user_by_id(req: HttpRequest) -> impl Responder {
    let data :Vec<User> = get_users_from_json();
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let user = data.into_iter().find(|a| a.id == id).unwrap();
    let serialized = serde_json::to_string(&user).unwrap();
    format!("{}", serialized)
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

fn get_users_from_json() -> Vec<User> {
    let users_list = r#"[
        {
          "id":1,
          "email":"1@gmail.com"
        },
        {
          "id":2,
          "email":"2@gmail.com"
        },
        {
          "id":3,
          "email":"3@gmail.com"
        },
        {
          "id":4,
          "email":"4@gmail.com"
        },
        {
          "id":5,
          "email":"5@gmail.com"
        },
        {
          "id":6,
          "email":"6@gmail.com"
        },
        {
          "id":7,
          "email":"7@gmail.com"
        },
        {
          "id":8,
          "email":"8@gmail.com"
        },
        {
          "id":9,
          "email":"9@gmail.com"
        },
        {
          "id":10,
          "email":"10@gmail.com"
        }
        ]"#;

    return serde_json::from_str(users_list).unwrap();
}