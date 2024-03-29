use std::sync::Arc;

use uuid::Uuid;

use crate::database::{cache::Cache, entities, loaders, PoolConnection};

use super::DataError;

pub async fn platform_role_by_user_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::platform_role::Entity>,
    user_id: Uuid,
) -> Result<Vec<Arc<entities::platform_role::Entity>>, DataError> {
    let ids_to_load = loaders::cacheless::find_many_ids_related_associative::<
        entities::user::Entity,
        entities::platform_role::Entity,
        entities::user_role::Entity,
    >(db_connection, user_id)
    .await?;

    Ok(loaders::cached::find_many_cached(cache, db_connection, Some(ids_to_load)).await?)
}

pub async fn platform_roles_by_ids(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::platform_role::Entity>,
    platform_role_ids: Option<Vec<Uuid>>,
) -> Result<Vec<Arc<entities::platform_role::Entity>>, DataError> {
    Ok(loaders::cached::find_many_cached(cache, db_connection, platform_role_ids).await?)
}
