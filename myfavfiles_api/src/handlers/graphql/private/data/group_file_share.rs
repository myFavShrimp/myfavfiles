use std::sync::Arc;

use uuid::Uuid;

use crate::database::{cache::Cache, entities, loaders, PoolConnection};

use super::DataError;

pub async fn group_file_shares_by_user_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group_file_share::Entity>,
    user_id: Uuid,
) -> Result<Vec<Arc<entities::group_file_share::Entity>>, DataError> {
    let ids_to_load = loaders::cacheless::find_many_ids_related::<
        entities::user::Entity,
        entities::group_file_share::Entity,
    >(db_connection, user_id)
    .await?;

    Ok(loaders::cached::find_many_cached(cache, db_connection, Some(ids_to_load)).await?)
}

pub async fn group_file_shares_by_group_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group_file_share::Entity>,
    group_id: Uuid,
) -> Result<Vec<Arc<entities::group_file_share::Entity>>, DataError> {
    let ids_to_load = loaders::cacheless::find_many_ids_related::<
        entities::group::Entity,
        entities::group_file_share::Entity,
    >(db_connection, group_id)
    .await?;

    Ok(loaders::cached::find_many_cached(cache, db_connection, Some(ids_to_load)).await?)
}
