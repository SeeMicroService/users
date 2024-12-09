use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FileStorage {
    type Error;
    type Rows;
    
    async fn add_file(&self, user_id: &Uuid, file: &str) -> Result<(), Self::Error>;

    async fn get_filenames(&self, user_id: &Uuid) -> Result<Self::Rows, Self::Error>;
    
    async fn delete_file(&self, user_id: &Uuid, file: &str) -> Result<(), Self::Error>;
}