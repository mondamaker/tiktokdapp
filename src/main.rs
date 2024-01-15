//cargo install cargo-watch
//cargo watch -x check
//cargo watch -x check -x test -x run
//cargo check
//cargo test

//curl -v http://127.0.0.1:8000
use zero2prod::run;
//use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //run().await
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run()?.await
}

/*
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hi_hello))
            .route("/{name}", web::get().to(hi_hello))
            .route("/health_check1", web::get().to(health_check1))
            .route("/health_check2", web::get().to(health_check2))
            .route("/health_check3", web::get().to(health_check3))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}


fn tokio() -> std::io::Result<()> {
    let body = async move {
        HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(hi_hello))
                .route("/{name}", web::get().to(hi_hello))
        })
            .bind("127.0.0.1:8000")?
            .run()
            .await
    };
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}*/