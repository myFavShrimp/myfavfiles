pub mod cached;
pub mod cacheless;

#[derive(thiserror::Error, Debug)]
pub enum LoaderError {
    #[error("sqlx error - {0}")]
    SqlxError(#[from] sqlx::Error),
}
