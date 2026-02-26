use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello from Rust API 🚀"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello));

    println!("Server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}