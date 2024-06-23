use axum::{routing::get, Router};

pub fn init() -> Router {
    Router::new().route("/", get(root_handler))
}

async fn root_handler() -> String {
    "Hello World".to_string()
}
