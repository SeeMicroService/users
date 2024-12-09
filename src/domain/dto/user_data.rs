use crate::domain::models::{Role, User};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserData {
    pub login: String,
    pub password: String,
    pub role: Role,
}

impl UserData {
    pub fn into_user(self, id: Uuid) -> User {
        User {
            id,
            login: self.login,
            password: self.password,
            role: self.role,
        }
    }
}
