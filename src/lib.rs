mod routes;
mod handlers;
pub async fn run() {
    let app = routes::create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}