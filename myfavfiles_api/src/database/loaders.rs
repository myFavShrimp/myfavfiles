pub mod cached;
pub mod cacheless;

#[derive(thiserror::Error, Debug)]
pub enum LoaderError {
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
}
