use crate::domain::dto::UserData;
use crate::domain::models::User;
use crate::domain::types::UserRepository;
use axum::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait UserService {
    type Error;

    async fn add(
        &self,
        credentials: UserData,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<Uuid, Self::Error>;

    async fn change_password(
        &self,
        id: &Uuid,
        password: String,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<(), Self::Error>;

    async fn remove(
        &self,
        id: &Uuid,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<(), Self::Error>;

    async fn info(
        &self,
        of: &Uuid,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<User, Self::Error>;

    async fn info_by_credentials(
        &self,
        login: &str,
        password: &str,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<User, Self::Error>;
}
