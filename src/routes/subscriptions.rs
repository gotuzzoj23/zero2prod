use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Always return a 200 OK
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
