use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
        // Define more routes here
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}