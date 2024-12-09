use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Credentials {
    pub id: Option<Uuid>,
    pub login: Option<String>,
    pub password: Option<String>,
}