use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use mysql::prelude::TextQuery;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("192.168.11.35", 21135))?
        .run()
        .await
}