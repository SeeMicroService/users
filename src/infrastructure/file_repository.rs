use crate::domain::{
    interfaces,
    types::{Error, FileStorage},
};
use axum::async_trait;
use std::sync::Arc;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct FileRepository {
    database: Arc<FileStorage<Vec<Row>, Error>>,
}

impl FileRepository {
    pub fn new(database: Arc<FileStorage<Vec<Row>, Error>>) -> FileRepository {
        Self { database }
    }
}

#[async_trait]
impl interfaces::FileRepository for FileRepository {
    type Error = Error;

    async fn add_file(&self, user_id: &Uuid, file: &str) -> Result<(), Self::Error> {
        self.database.add_file(user_id, file).await
    }

    async fn get_filenames(&self, id: &Uuid) -> Result<Vec<String>, Self::Error> {
        let rows = self.database.get_filenames(id).await?;
        let mut files = Vec::with_capacity(rows.len());
        for row in &rows {
            files.push(row.get("filename"));
        }
        Ok(files)
    }

    async fn delete_file(&self, user_id: &Uuid, file: &str) -> Result<(), Self::Error> {
        self.database.delete_file(user_id, file).await
    }
}
