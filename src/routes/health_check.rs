use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
