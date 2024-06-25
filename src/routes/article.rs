use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{models::article::Article, schemas::article::ArticleRequestData, AppState};

pub async fn create_handler(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<ArticleRequestData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // titleの空白を-に置換する
    let slug = payload.article.title.to_lowercase().replace(' ', "-");
    let article_record = sqlx::query_as!(
        Article,
        r#"
        INSERT INTO articles (slug, title, description, body)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        &slug,
        &payload.article.title,
        &payload.article.description,
        &payload.article.body
    )
    .fetch_one(&data.db)
    .await;

    match article_record {
        Ok(record) => {
            // Construct the response JSON using the record data.
            let json_response = json!({"article": record });
            Ok((StatusCode::CREATED, Json(json_response)))
        }
        Err(error) => {
            // Handle the error, e.g., by logging it and returning an error response.
            let error_json = json!({ "errors": { "body": [error.to_string()] } });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_json)))
        }
    }
}
