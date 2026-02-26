use axum::{
    routing::{get, post},
    Router,
};

mod handler;
mod service;
mod repository;
mod model;

use handler::user_handler::{get_users, create_user};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(get_users).post(create_user));

    println!("Server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}