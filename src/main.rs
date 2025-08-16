mod handlers;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::greet)) // Using the greet function from the handlers module
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}