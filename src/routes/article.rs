use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::article::{Article},
    schemas::article::{ArticleRequestData, ArticlesResponseSchema, FilterOptions, ParamOptions},
    AppState,
};

pub async fn create_handler(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<ArticleRequestData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let slug = payload.article.title.replace(' ', "-");
    let result = sqlx::query_as!(
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

    match result {
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

pub async fn get_handler(
    State(data): State<Arc<AppState>>,
    Path(param): Path<ParamOptions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query_as!(
        Article,
        r#"
        SELECT * FROM articles
        WHERE slug = $1
        "#,
        &param.slug
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(record) => {
            let json_response = json!({"article": record });
            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(error) => {
            let error_json = json!({ "errors": { "body": [error.to_string()] } });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_json)))
        }
    }
}

pub async fn get_list_handler(
    State(data): State<Arc<AppState>>,
    Query(param): Query<FilterOptions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let offset = param.offset.unwrap_or(0);
    let limit = param.limit.unwrap_or(20);

    let result = sqlx::query_as!(
        Article,
        r#"
        SELECT * FROM articles
        ORDER BY created_at DESC
        OFFSET $1
        LIMIT $2
        "#,
        offset as i64,
        limit as i64
    )
    .fetch_all(&data.db)
    .await;

    match result {
        Ok(articles) => {
            let articles_count = articles.len();
            let articles_response = ArticlesResponseSchema {
                articles,
                articles_count,
            };
            Ok((StatusCode::OK, Json(articles_response)))
        },
        Err(_) => {
            let error_json = json!({ "errors": { "body": ["Could not fetch articles"] } });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_json)))
        }
    }
}
