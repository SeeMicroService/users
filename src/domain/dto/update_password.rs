use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub id: Uuid,
    pub password: String,
}