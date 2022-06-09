use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Always return a 200 OK
pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    // Let's generate a random unique identifier
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // 'info_span' creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    // Using 'enter' in an async function is a recipe for disaster!
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database",);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use 'get_ref' to get an immutable reference to the 'PgConnection'
    // Wrapped by 'web::data'.
    .execute(connection_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
