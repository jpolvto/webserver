use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::http::StatusCode;
use bson::{doc, Document};
use mongodb::{Client, Collection, Database};
use mongodb::options::ClientOptions;
use serde_json;
use futures_util::stream::StreamExt;

pub struct AppState {
    pub data: Database,
}

pub async fn all_users(data: web::Data<AppState>) -> impl Responder {
    let user_collection: Collection<Document> = data.data.collection("users");
    let mut cursor = user_collection.find(doc! {}, None).await.unwrap();

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
    let user_collection: Collection<Document> = data.data.collection("users");
    let result = user_collection.find_one(doc! { "id":  id }, None).await.unwrap();

    let mut results: Vec<Document> = Vec::new();

    if let Some(doc) = result {
        results.push(doc);
    }

    HttpResponse::Ok().json(results)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_url = "mongodb://127.0.0.1:27017";
    let database_name = "actix-web";
    let options = ClientOptions::parse(&database_url).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database(&database_name);

    let data = web::Data::new(AppState {
        data: db,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // <- register the created data
            .route("/users", web::get().to(all_users))
            .route("/users/{id}", web::get().to(get_user_by_id))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}