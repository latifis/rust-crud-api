use sqlx::{Pool, Postgres};
use crate::dto::create_user_request::CreateUserRequest;
use crate::dto::update_user_request::UpdateUserRequest;
use crate::model::user::User;

pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, name FROM users"
    )
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn save(
    pool: &Pool<Postgres>,
    user: CreateUserRequest
) -> Result<User, sqlx::Error> {

    let rec = sqlx::query!(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        user.name
    )
        .fetch_one(pool)
        .await?;

    Ok(User {
        id: rec.id,
        name: rec.name,
    })
}

pub async fn update(
    pool: &Pool<Postgres>,
    user: UpdateUserRequest
) -> Result<User, sqlx::Error> {

    let rec = sqlx::query!(
        "UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name",
        user.name,
        user.id
    )
        .fetch_one(pool)
        .await?;

    Ok(User {
        id: rec.id,
        name: rec.name,
    })
}