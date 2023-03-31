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
    #[error("error in data loading")]
    LoaderError(#[from] LoaderError),
}
