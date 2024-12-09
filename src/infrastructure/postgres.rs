use crate::domain::interfaces::{FileStorage, UserStorage};
use crate::domain::models::User;
use crate::domain::types::{self, Error};
use axum::async_trait;
use deadpool_postgres::{Manager, Pool, Transaction};
use std::ops::DerefMut;
use std::str::FromStr;
use tokio_postgres as postgres;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct Postgres {
    pool: Pool,
}

mod migration {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

impl Postgres {
    pub async fn new(uri: &str) -> types::Result<Postgres> {
        let config = postgres::Config::from_str(uri)?;
        let manager = Manager::new(config, tokio_postgres::NoTls);
        let pool = Pool::builder(manager).build()?;
        let mut connection = pool.get().await?;
        migration::migrations::runner()
            .run_async(connection.deref_mut().deref_mut())
            .await?;
        Ok(Postgres { pool })
    }

    async fn commit_or_rollback<T>(
        transaction: Transaction<'_>,
        result: Result<T, tokio_postgres::Error>,
    ) -> types::Result<T> {
        match result {
            Ok(data) => {
                transaction.commit().await?;
                Ok(data)
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err)?
            }
        }
    }
}

#[async_trait]
impl UserStorage for Postgres {
    type Error = Error;
    type Rows = Vec<Row>;

    async fn add(&self, user: &User) -> types::Result<()> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let statement = transaction
            .prepare_cached("INSERT INTO Users VALUES ($1, $2, $3, $4)")
            .await?;
        let result = transaction
            .execute(
                &statement,
                &[&user.id, &user.login, &user.password, &user.role],
            )
            .await;
        Self::commit_or_rollback(transaction, result).await?;
        Ok(())
    }

    async fn remove(&self, id: &Uuid) -> types::Result<()> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let statement = transaction
            .prepare_cached("DELETE FROM Users WHERE id = $1")
            .await?;
        let result = transaction.execute(&statement, &[&id]).await;
        Self::commit_or_rollback(transaction, result).await?;
        Ok(())
    }

    async fn get(&self, id: &Uuid) -> types::Result<Self::Rows> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let statement = transaction
            .prepare_cached("SELECT * FROM Users WHERE id = $1")
            .await?;
        let result = transaction.query(&statement, &[&id]).await;
        Self::commit_or_rollback(transaction, result).await
    }

    async fn get_by_login(&self, login: &str, password: &str) -> Result<Self::Rows, Self::Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let statement = transaction
            .prepare_cached("SELECT * FROM Users WHERE login = $1 AND password = $2")
            .await?;
        let result = transaction.query(&statement, &[&login, &password]).await;
        Self::commit_or_rollback(transaction, result).await
    }
    async fn update(&self, user: &User) -> types::Result<()> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let statement = transaction
            .prepare_cached("UPDATE Users SET login = $1, password = $2, role = $3 WHERE id = $4")
            .await?;
        let result = transaction
            .execute(
                &statement,
                &[&user.login, &user.password, &user.role, &user.id],
            )
            .await;
        Self::commit_or_rollback(transaction, result).await?;
        Ok(())
    }
}

#[async_trait]
impl FileStorage for Postgres {
    type Error = Error;
    type Rows = Vec<Row>;

    async fn add_file(&self, user_id: &Uuid, file: &str) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let statement = transaction
            .prepare_cached("INSERT INTO Files VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .await?;
        let result = transaction.query(&statement, &[&user_id, &file]).await;
        Self::commit_or_rollback(transaction, result).await?;
        Ok(())
    }

    async fn get_filenames(&self, user_id: &Uuid) -> Result<Self::Rows, Self::Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let statement = transaction
            .prepare_cached("SELECT * FROM Files WHERE user_id = $1")
            .await?;
        let result = transaction.query(&statement, &[&user_id]).await;
        Self::commit_or_rollback(transaction, result).await
    }

    async fn delete_file(&self, user_id: &Uuid, file: &str) -> Result<(), Self::Error> {
        let mut connection = self.pool.get().await?;
        let transaction = connection.transaction().await?;
        let statement = transaction
            .prepare_cached("DELETE FROM Files WHERE user_id = $1 AND filename = $2")
            .await?;
        let result = transaction.query(&statement, &[&user_id, &file]).await;
        Self::commit_or_rollback(transaction, result).await?;
        Ok(())
    }
}
