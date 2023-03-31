use std::sync::Arc;

use uuid::Uuid;

use crate::database::{cache::Cache, entities, loaders, PoolConnection};

use super::DataError;

pub async fn group_by_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group::Entity>,
    group_id: Uuid,
) -> Result<Option<Arc<entities::group::Entity>>, DataError> {
    Ok(
        loaders::cached::find_many_cached(cache, db_connection, Some(vec![group_id]))
            .await?
            .pop(),
    )
}

// pub async fn create_group(
//     db_connection: &mut PoolConnection,
//     cache: Cache<entities::group::Entity>,
//     new_group: GroupCreaionInput,
// ) -> Arc<entities::group::Entity> {
// }
