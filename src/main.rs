use  actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
// chekc

// healeth check handler
async fn healt_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!",&name)
}


#[tokio::main]
async fn  main() -> std::io::Result<()> {
    println!("Server running at port 8000");
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet)).route("/{name}", web::get().to(greet)).route("/health_check", web::get().to(healt_check))
    }).bind("127.0.0.1:8000")?.run().await
    
}
