use sqlx::{Pool, Postgres};

pub async fn connect_db(database_url: &str) -> Pool<Postgres> {
    Pool::<Postgres>::connect(database_url)
        .await
        .expect("Failed to connect DB")
}