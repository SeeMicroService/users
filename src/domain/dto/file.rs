use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FileDto {
    pub id: Uuid,
    pub filename: String,
}
