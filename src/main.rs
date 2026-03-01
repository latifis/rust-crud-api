use axum::{
    routing::{get, post},
    Router,
};
use std::env;

mod handler;
mod service;
mod repository;
mod model;
mod config;
pub mod dto;

use handler::user_handler::{get_users, create_user, update_user};
use config::db::connect_db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let db = connect_db(&db_url).await;

    let app = Router::new()
        .route("/users", get(get_users).post(create_user).put(update_user))
        .with_state(db);

    println!("Server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}