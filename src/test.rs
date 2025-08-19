use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use env_logger;
use actix_web::{put, delete, web::Json};
use actix_web::web::Query;
use actix_web::HttpRequest;
use actix_web::{http::StatusCode, http::header};
use actix_web::{ResponseError, error::InternalError};
use derive_more::{Display, From};

mod kafka;
use crate::kafka::produce_message;

#[derive(Debug, Display, From)]
pub enum CustomError {
    #[display( "Internal Server Error")]
    InternalServerError,

    #[display( "Bad Request: {}", _0)]
    BadRequest(String),
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::InternalServerError => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
            CustomError::BadRequest(message) => HttpResponse::BadRequest().body(message.clone()),
        }
    }
}

#[get("/error")]
async fn generate_error() -> Result<HttpResponse, CustomError> {
    Err(CustomError::InternalServerError)
}

#[get("/error2")]
async fn generate_error2() -> Result<HttpResponse, actix_web::Error> {
    Err(InternalError::new(
        "Specific error message",
        StatusCode::BAD_REQUEST,
    ).into())
}
#[get("/status")]
async fn status_codes() -> impl Responder {
    HttpResponse::build(StatusCode::CREATED)
        .insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8))
        .body("Resource created successfully!")
}

#[get("/headers")]
async fn headers(req: HttpRequest) -> impl Responder {
    let mut headers_string = String::new();
    for (name, value) in req.headers().iter() {
        headers_string.push_str(&format!("{}: {}\n", name, value.to_str().unwrap_or("unknown")));
    }
    HttpResponse::Ok().body(format!("Request Headers:\n{}", headers_string))
}
#[derive(Deserialize)]
struct Info {
    username: String,
}
#[get("/query")]
async fn query_params(info: Query<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}
#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: i32,
    name: String,
}
#[put("/item/{item_id}")]
async fn update_item(item_id: web::Path<i32>, item: web::Json<Item>) -> impl Responder {
    let item_id = item_id.into_inner();
    println!("Updating item with ID: {}", item_id);
    println!("Received item: {:?}", item);
    HttpResponse::Ok().body(format!("Updated item with ID: {}", item_id))
}
#[delete("/item/{item_id}")]
async fn delete_item(item_id: web::Path<i32>) -> impl Responder {
    let item_id = item_id.into_inner();
    println!("Deleting item with ID: {}", item_id);
    HttpResponse::Ok().body(format!("Deleted item with ID: {}", item_id))
}
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/user/{user_id}")]
async fn get_user(user_id: web::Path<i32>) -> impl Responder {
    let user_id = user_id.into_inner();
    let user = User {
        id: user_id,
        name: "Example User".to_string(),
    };

    HttpResponse::Ok().json(user)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    unsafe{
    std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    HttpServer::new(|| {
       App::new()
    .service(hello)
    .service(echo)
    .route("/hey", web::get().to(manual_hello))
    .service(get_user)
    .service(update_item)
    .service(delete_item)
    .service(query_params)
    .service(headers)
    .service(status_codes)
    .service(generate_error)
    .service(generate_error2)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}