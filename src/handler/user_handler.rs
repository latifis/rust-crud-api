use axum::Json;
use crate::model::user::User;
use crate::service::user_service;

pub async fn get_users() -> Json<Vec<User>> {
    let users = user_service::get_all_users();
    Json(users)
}

pub async fn create_user(Json(payload): Json<User>) -> Json<User> {
    let user = user_service::create_user(payload);
    Json(user)
}