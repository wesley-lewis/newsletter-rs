use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, dev::Server};
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(address: &str) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health_check", web::get().to(health_check))
    })
    .bind(address)?
    .run();

    Ok(server)
}
