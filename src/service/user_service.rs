use crate::model::user::User;
use crate::repository::user_repository;

pub fn get_all_users() -> Vec<User> {
    user_repository::find_all()
}

pub fn create_user(user: User) -> User {
    user_repository::save(user)
}