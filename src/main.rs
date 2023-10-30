
use actix_web::{HttpServer, App, web, Responder, HttpResponse};


async fn health_check() -> impl Responder{
    println!("hello world");
    HttpResponse::Ok().body("hello world")
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().route("/", web::get().to(health_check))
    }).workers(4).bind("127.0.0.1:8000")?.run().await

}
