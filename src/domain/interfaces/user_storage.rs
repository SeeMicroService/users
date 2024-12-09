use axum::async_trait;
use uuid::Uuid;
use crate::domain::models::User;

#[async_trait]
pub trait UserStorage {
    
    type Error;
    type Rows;
    
    async fn add(&self, user: &User) -> Result<(), Self::Error>;
    async fn remove(&self, id: &Uuid) -> Result<(), Self::Error>;
    async fn get(&self, id: &Uuid) -> Result<Self::Rows, Self::Error>;
    
    async fn get_by_login(&self, login: &str, password: &str) -> Result<Self::Rows, Self::Error>;
    
    async fn update(&self, user: &User) -> Result<(), Self::Error>;
}