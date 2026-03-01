use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::{Pool, Postgres};

use crate::model::user::User;
use crate::service::user_service;

pub async fn get_users(
    State(pool): State<Pool<Postgres>>
) -> Result<Json<Vec<User>>, StatusCode> {

    let users = user_service::get_all_users(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<User>,
) -> Json<User> {
    let user = user_service::create_user(&pool, payload).await;
    Json(user)
}