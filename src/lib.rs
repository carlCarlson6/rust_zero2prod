use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SubscribeFormData {
    email: String,
    name: String,
}

async fn subscribe(form: web::Form<SubscribeFormData>) -> HttpResponse {
    println!(
        "request recieved on subscriptions for {} - {}",
        form.email, form.name
    );
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
