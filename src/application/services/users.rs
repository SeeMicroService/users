use crate::domain::dto::UserData;
use crate::domain::interfaces;
use crate::domain::models::User;
use crate::domain::types::{Error, UserRepository};
use axum::async_trait;
use std::io;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService;

#[async_trait]
impl interfaces::UserService for UserService {
    type Error = Error;

    async fn add(
        &self,
        credentials: UserData,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<Uuid, Self::Error> {
        let id = Uuid::new_v4();
        let user = credentials.into_user(id);
        match repo.get(&user.id).await {
            Ok(Some(_)) => Err(io::Error::from(io::ErrorKind::AlreadyExists).into()),
            Ok(None) => {
                repo.push(&user).await?;
                Ok(user.id)
            }
            Err(e) => Err(e),
        }
    }

    async fn change_password(
        &self,
        id: &Uuid,
        password: String,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<(), Self::Error> {
        let user = repo.get(id).await?;
        match user {
            Some(mut user) => {
                user.password = password;
                repo.update(&user).await
            }
            None => Err(io::Error::from(io::ErrorKind::NotFound).into()),
        }
    }

    async fn remove(
        &self,
        id: &Uuid,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<(), Self::Error> {
        let user = repo.get(id).await?;
        match user {
            Some(_) => repo.delete(id).await,
            None => Err(io::Error::from(io::ErrorKind::NotFound).into()),
        }
    }

    async fn info(
        &self,
        of: &Uuid,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<User, Self::Error> {
        match repo.get(of).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(io::Error::from(io::ErrorKind::NotFound).into()),
            Err(e) => Err(e),
        }
    }

    async fn info_by_credentials(
        &self,
        login: &str,
        password: &str,
        repo: Arc<UserRepository<Self::Error>>,
    ) -> Result<User, Self::Error> {
        match repo.get_by_login(login, password).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(io::Error::from(io::ErrorKind::NotFound).into()),
            Err(e) => Err(e),
        }
    }
}
