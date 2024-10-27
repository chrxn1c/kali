use axum::Router;
use axum::routing::get;

pub async fn run() {
    let app = Router::new().route("/", get(|| async { "Hello world!" }));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}