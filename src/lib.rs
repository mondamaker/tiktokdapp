use std::net::TcpListener;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder};
use actix_web::dev::Server;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}
async fn subscribe_form(_form: web::Form<FormData>) -> HttpResponse {
    format!("Welcome {}!", _form.name);
    HttpResponse::Ok().finish()
}
async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn health_check1(req: HttpRequest) -> impl Responder {
    println!("todo health_check1");
    HttpResponse::Ok()
}

async fn health_check2(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().finish()
}

async fn health_check3(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok()
}

async fn hi_hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .route("/subscriptions_form", web::post().to(subscribe_form))
    })
        .bind("127.0.0.1:8888")?
        .run();
    // No .await here!
    Ok(server)
}

pub fn run_random(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions_form", web::post().to(subscribe_form))
    })
        .bind(address)?
        .run();
    // No .await here!
    Ok(server)
}


pub fn run_listener(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::get().to(subscribe))
            .route("/subscriptions_form", web::post().to(subscribe_form))
    })
        .listen(listener)?
        .run();
    Ok(server)
}



pub async fn run_all() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hi_hello))
            .route("/{name}", web::get().to(hi_hello))
            .route("/health_check", web::get().to(health_check))
            .route("/health_check1", web::get().to(health_check1))
            .route("/health_check2", web::get().to(health_check2))
            .route("/health_check3", web::get().to(health_check3))
            .route("/subscribe", web::get().to(subscribe))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}



/*
#[cfg(test)]
mod tests {
    use crate::health_check;
    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}*/