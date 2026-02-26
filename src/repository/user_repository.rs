use crate::model::user::User;

pub fn find_all() -> Vec<User> {
    vec![
        User { id: 1, name: "Budi".to_string() },
        User { id: 2, name: "Andi".to_string() },
    ]
}

pub fn save(user: User) -> User {
    user
}