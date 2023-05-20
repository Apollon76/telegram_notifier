use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};

async fn ping() -> HttpResponse {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/ping").route(web::post().to(ping)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}