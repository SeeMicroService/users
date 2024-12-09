use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::roles::Role;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub password: String,
    pub role: Role,
}