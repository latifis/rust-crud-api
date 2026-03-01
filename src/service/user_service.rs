use sqlx::{Pool, Postgres};
use crate::model::user::User;
use crate::repository::user_repository;

pub async fn get_all_users(
    pool: &Pool<Postgres>
) -> Result<Vec<User>, sqlx::Error> {
    user_repository::find_all(pool).await
}

pub async fn create_user(pool: &Pool<Postgres>, user: User) -> User {
    user_repository::save(pool, user).await
}