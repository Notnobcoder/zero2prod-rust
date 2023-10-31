//
// use actix_web::{HttpServer, App, web, Responder, HttpResponse};
//
//
// async fn health_check() -> impl Responder{
//     println!("hello world");
//     HttpResponse::Ok().body("hello world")
// }
//
// #[tokio::main]
// async fn main() -> std::io::Result<()>{
//     HttpServer::new(|| {
//         App::new().route("/", web::get().to(health_check))
//     }).workers(4).bind("127.0.0.1:8000")?.run().await
//
// }
//
//
use actix_web::{HttpServer, Responder, HttpResponse, App ,web};

async fn value2() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(value2))
    }).bind("127.0.0.1:8000")?.run().await
} 
