use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub id: i32,
    pub name: String,
}