use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SubscribeFormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<SubscribeFormData>) -> HttpResponse {
    println!(
        "request recieved on subscriptions for {} - {}",
        form.email, form.name
    );
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}
