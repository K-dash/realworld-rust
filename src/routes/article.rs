use std::sync::Arc;
use tracing::info;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::article::Article,
    schemas::article::{
        CreateArticleSchema, FilterOptions, ParamOptions, RequestArticleSchema,
        ResponseArticleSchema, ResponseArticlesSchema, UpdateArticleSchema,
    },
    AppState,
};

pub async fn create_handler(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<RequestArticleSchema<CreateArticleSchema>>,
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
            let json_response = ResponseArticleSchema { article: record };
            Ok((StatusCode::CREATED, Json(json_response)))
        }
        Err(error) => {
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
            let json_response = ResponseArticleSchema { article: record };
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
            let articles_response = ResponseArticlesSchema {
                articles,
                articles_count,
            };
            Ok((StatusCode::OK, Json(articles_response)))
        }
        Err(_) => {
            let error_json = json!({ "errors": { "body": ["Could not fetch articles"] } });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_json)))
        }
    }
}

pub async fn update_handler(
    State(data): State<Arc<AppState>>,
    Path(param): Path<ParamOptions>,
    Json(payload): Json<RequestArticleSchema<UpdateArticleSchema>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let get_result = sqlx::query_as!(
        Article,
        r#"
        SELECT * FROM articles
        WHERE slug = $1
        "#,
        &param.slug
    )
    .fetch_one(&data.db)
    .await;

    if get_result.is_err() {
        let error_response =
            serde_json::json!({"errors": { "body": ["Articles with that slug does not exist"] } });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let article = get_result.unwrap();

    // generate new slug
    let new_slug = if let Some(ref title) = payload.article.title {
        if title != &article.title {
            Some(title.replace(' ', "-"))
        } else {
            None
        }
    } else {
        None
    };

    info!("payload.title: {:?}", payload.article.title);
    info!("payload.description: {:?}", payload.article.description);

    let update_result = sqlx::query_as!(
        Article,
        r#"
        UPDATE articles
        SET title = COALESCE($2, title),
            description = COALESCE($3, description),
            body = COALESCE($4, body),
            slug = COALESCE($5, slug),
            updated_at = NOW()
        WHERE slug = $1
        RETURNING *
        "#,
        &param.slug,
        payload.article.title.as_deref(),
        payload.article.description.as_deref(),
        payload.article.body.as_deref(),
        new_slug.as_deref()
    )
    .fetch_one(&data.db)
    .await;

    match update_result {
        Ok(record) => {
            let json_response = ResponseArticleSchema { article: record };
            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(_) => {
            let error_response = json!({"errors": {"body": ["Could not update the article"]}});
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub async fn delete_handler(
    State(data): State<Arc<AppState>>,
    Path(param): Path<ParamOptions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        r#"
        DELETE FROM articles
        WHERE slug = $1
        "#,
    )
    .bind(&param.slug)
    .execute(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "errors": { "body": [e.to_string()] } })),
        )
    })?;

    if result.rows_affected() == 0 {
        let error_response = json!({"errors": {"body": ["Article with that slug does not exist"]}});
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
