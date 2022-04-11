use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest};

pub async fn health_check() -> HttpResponse { 
    HttpResponse::Ok().finish()
}