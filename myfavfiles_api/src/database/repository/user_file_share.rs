use std::sync::Arc;

use uuid::Uuid;

use crate::database::{cache::Cache, entities, loaders, PoolConnection};

use super::DataError;

pub async fn user_file_shares_by_user_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::user_file_share::Entity>,
    user_id: Uuid,
) -> Result<Vec<Arc<entities::user_file_share::Entity>>, DataError> {
    Ok(loaders::cached::find_many_cached(cache, db_connection, Some(vec![user_id])).await?)
}
