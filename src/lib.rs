use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> HttpResponse { 
    HttpResponse::Ok().finish()
}

// Running main app
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| { 
        App::new()
            .route("/health_check", web::get().to(health_check))
            })
            .listen(listener)?
            .run();
    Ok(server)
}

// Get random port to listen to for user
pub fn create_listerner() -> TcpListener {
    let listener =  TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");
    return listener
}

