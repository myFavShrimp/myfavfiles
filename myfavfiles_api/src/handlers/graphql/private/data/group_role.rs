use std::sync::Arc;

use uuid::Uuid;

use crate::database::{cache::Cache, entities, loaders, PoolConnection};

use super::DataError;

pub async fn group_roles_by_group_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group_role::Entity>,
    group_id: Uuid,
) -> Result<Vec<Arc<entities::group_role::Entity>>, DataError> {
    let ids_to_load = loaders::cacheless::find_many_ids_related::<
        entities::group::Entity,
        entities::group_role::Entity,
    >(db_connection, group_id)
    .await?;

    Ok(loaders::cached::find_many_cached(cache, db_connection, Some(ids_to_load)).await?)
}

pub async fn group_roles_by_ids(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group_role::Entity>,
    group_ids: Option<Vec<Uuid>>,
) -> Result<Vec<Arc<entities::group_role::Entity>>, DataError> {
    Ok(loaders::cached::find_many_cached(cache, db_connection, group_ids).await?)
}

pub async fn group_roles_by_group_member_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group_role::Entity>,
    group_member_id: Uuid,
) -> Result<Vec<Arc<entities::group_role::Entity>>, DataError> {
    let ids_to_load = loaders::cacheless::find_many_ids_related_associative::<
        entities::group_member::Entity,
        entities::group_role::Entity,
        entities::group_member_role::Entity,
    >(db_connection, group_member_id)
    .await?;

    Ok(loaders::cached::find_many_cached(cache, db_connection, Some(ids_to_load)).await?)
}
