mod models;

use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use crate::models::User;

pub struct AppStateWithData {
    pub data : Mutex <Vec<User>>,
}

pub async fn get_users(data: web::Data<AppStateWithData>) -> impl Responder {
    let serialized = serde_json::to_string(&*data.data.lock().unwrap()).unwrap();
    format!("{}", serialized)
}

pub async fn get_user_by_id(req: HttpRequest, data: web::Data<AppStateWithData>) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let parsed_data = &*data.data.lock().unwrap();
    let user = parsed_data.into_iter().find(|a| a.id == id).unwrap();
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

    // Load data from json file
    let mut file = File::open("src/data.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let json_data:Mutex<Vec<User>> = serde_json::from_str(&mut buff).unwrap();

    let data = web::Data::new(AppStateWithData {
        data: json_data,
    });

    // Start http server
    HttpServer::new(move || {
    App::new()
        .app_data(data.clone()) // <- register the created data
        .route("/users", web::get().to(get_users))
        .route("/users/{id}", web::get().to(get_user_by_id))
        .route("/users", web::get().to(add_user))
        .route("/users/{id}", web::get().to(delete_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
