use actix_web::{
    web::{self},
    App, HttpRequest, HttpServer, Responder,
};
use std::io::Error;

async fn greet(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("Word");
    format!("hello {}!", &name)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello world! - starting server");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
