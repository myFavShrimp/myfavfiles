use crate::database::loaders::LoaderError;

pub mod group;
pub mod group_file_share;
pub mod group_member;
pub mod group_role;
pub mod platform_role;
pub mod user;
pub mod user_file_share;

#[derive(thiserror::Error, Debug)]
pub enum DataError {
    #[error("error in data loading - {0}")]
    LoaderError(#[from] LoaderError),
    #[error("sqlx error - {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("error in query creation - {0}")]
    SeaQueryError(#[from] sea_query::error::Error),
}
