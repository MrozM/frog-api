use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    let address = req.peer_addr().unwrap().ip();
    let message = format!("hi! your ip: {:?}", address) ;
    HttpResponse::Ok().body(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(("192.168.11.35", 21135))?.run().await
}