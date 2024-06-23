mod routes;

use tokio::net::TcpListener;
use routes::home::init as home_init;

#[tokio::main]
async fn main() {
    // ルートエンドポイントの初期化
    let app = home_init();
    
    // サーバーを起動
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
