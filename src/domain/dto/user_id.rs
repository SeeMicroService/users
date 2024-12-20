use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserId {
    pub id: Uuid,
}
