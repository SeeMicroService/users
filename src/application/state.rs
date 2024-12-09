use std::sync::Arc;
use crate::domain::types::{Error, UserRepository, UserService, FileRepository};

#[derive(Clone)]
pub struct State {
    pub user_repo: Arc<UserRepository<Error>>,
    pub user_service: Arc<UserService<Error>>,
    pub file_repo: Arc<FileRepository<Error>>,
}