mod models;
mod routes;
mod schemas;

use axum::{routing::get, routing::post, Router};
use dotenv::dotenv;
use routes::article::create_handler;
use routes::healthcheck::health_check_handler;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    // tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug,axum_sandbox=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Main Server Start");

    // load env file
    dotenv().ok();

    // connect to database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            tracing::error!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // register routes
    let app = Router::new()
        .route("/", get(health_check_handler))
        .route("/articles", post(create_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    // run server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
