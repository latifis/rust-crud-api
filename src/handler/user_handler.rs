use axum::{extract::State, Json};
use sqlx::{Pool, Postgres};

use crate::model::user::User;
use crate::service::user_service;

pub async fn get_users(
    State(pool): State<Pool<Postgres>>
) -> Json<Vec<User>> {
    let users = user_service::get_all_users(&pool).await;
    Json(users)
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<User>,
) -> Json<User> {
    let user = user_service::create_user(&pool, payload).await;
    Json(user)
}