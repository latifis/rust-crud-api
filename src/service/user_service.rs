use sqlx::{Pool, Postgres};
use crate::dto::create_user_request::CreateUserRequest;
use crate::dto::update_user_request::UpdateUserRequest;
use crate::model::user::User;
use crate::repository::user_repository;

pub async fn get_all_users(
    pool: &Pool<Postgres>
) -> Result<Vec<User>, sqlx::Error> {
    user_repository::find_all(pool).await
}

pub async fn create_user(
    pool: &Pool<Postgres>,
    user: CreateUserRequest
) -> Result<User, sqlx::Error> {

    user_repository::save(pool, user).await
}

pub async fn update_user(
    pool: &Pool<Postgres>,
    user: UpdateUserRequest
) -> Result<User, sqlx::Error> {

    user_repository::update(pool, user).await
}