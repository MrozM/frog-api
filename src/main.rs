use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    let message = println!("Hello, your ip address is: {:?}", req.peer_addr());
    HttpResponse::Ok().body(message);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(("192.168.11.35", 21135))?.run().await
}