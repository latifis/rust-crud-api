use sqlx::{Pool, Postgres};
use crate::model::user::User;

pub async fn find_all(pool: &Pool<Postgres>) -> Vec<User> {
    sqlx::query_as!(
        User,
        "SELECT id, name FROM users"
    )
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn save(pool: &Pool<Postgres>, user: User) -> User {
    let rec = sqlx::query!(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        user.name
    )
        .fetch_one(pool)
        .await
        .unwrap();

    User {
        id: rec.id,
        name: rec.name,
    }
}