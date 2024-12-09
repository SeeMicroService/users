use axum::async_trait;
use uuid::Uuid;
use crate::domain::models::User;

#[async_trait]
pub trait UserRepository {
    type Error;
    
    async fn push(&self, user: &User) -> Result<(), Self::Error>;
    async fn delete(&self, id: &Uuid) -> Result<(), Self::Error>;
    
    async fn get(&self, id: &Uuid) -> Result<Option<User>, Self::Error>;
    
    async fn get_by_login(&self, login: &str, password: &str) -> Result<Option<User>, Self::Error>;
    
    async fn update(&self, user: &User) -> Result<(), Self::Error>;
}