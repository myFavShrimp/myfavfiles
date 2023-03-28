use std::sync::Arc;

use uuid::Uuid;

use crate::database::{cache::Cache, entities, loaders, PoolConnection};

pub async fn group_by_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group::Entity>,
    group_id: Uuid,
) -> Option<Arc<entities::group::Entity>> {
    loaders::cached::find_many_cached(cache, db_connection, Some(vec![group_id]))
        .await
        .pop()
}
