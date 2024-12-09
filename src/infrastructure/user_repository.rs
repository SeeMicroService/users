use crate::domain::{
    interfaces,
    models::User,
    types::{Error, UserStorage},
};
use axum::async_trait;
use std::io;
use std::sync::Arc;
use tokio_postgres::error::SqlState;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct UserRepository {
    database: Arc<UserStorage<Vec<Row>, Error>>,
}

impl UserRepository {
    pub fn new(database: Arc<UserStorage<Vec<Row>, Error>>) -> Self {
        Self { database }
    }

    fn fill_user(rows: Vec<Row>) -> Option<User> {
        if rows.is_empty() {
            None
        } else {
            Some(User {
                id: rows[0].get("id"),
                login: rows[0].get("login"),
                password: rows[0].get("password"),
                role: rows[0].get("role"),
            })
        }
    }
}

#[async_trait]
impl interfaces::UserRepository for UserRepository {
    type Error = Error;

    async fn push(&self, user: &User) -> Result<(), Self::Error> {
        match self.database.add(user).await {
            Ok(_) => Ok(()),
            Err(error) => {
                if let Some(error) = error.downcast_ref::<tokio_postgres::Error>() {
                    if let Some(db_error) = error.as_db_error() {
                        if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                            return Err(io::Error::from(io::ErrorKind::AlreadyExists).into());
                        }
                    }
                }
                Err(error)
            }
        }
    }

    async fn delete(&self, id: &Uuid) -> Result<(), Self::Error> {
        self.database.remove(id).await
    }

    async fn get(&self, id: &Uuid) -> Result<Option<User>, Self::Error> {
        let rows = self.database.get(id).await?;
        Ok(Self::fill_user(rows))
    }

    async fn get_by_login(&self, login: &str, password: &str) -> Result<Option<User>, Self::Error> {
        let rows = self.database.get_by_login(login, password).await?;
        Ok(Self::fill_user(rows))
    }

    async fn update(&self, user: &User) -> Result<(), Self::Error> {
        self.database.update(user).await
    }
}
