use crate::domain::interfaces;

pub(crate) type Error = Box<dyn std::error::Error + Sync + Send>;
pub(crate) type UserRepository<E> = dyn interfaces::UserRepository<Error = E> + Send + Sync;
pub(crate) type UserStorage<R, E> = dyn interfaces::UserStorage<Rows = R, Error = E> + Send + Sync;

pub(crate) type UserService<E> = dyn interfaces::UserService<Error = E> + Send + Sync;

pub(crate) type FileStorage<R, E> = dyn interfaces::FileStorage<Rows = R, Error = E> + Send + Sync;

pub(crate) type FileRepository<E> = dyn interfaces::FileRepository<Error = E> + Send + Sync;

pub(crate) type Result<T> = std::result::Result<T, Error>;