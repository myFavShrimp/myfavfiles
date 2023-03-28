use std::sync::Arc;

use uuid::Uuid;

use crate::database::{cache::Cache, entities, loaders, PoolConnection};

pub async fn user_by_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::user::Entity>,
    user_id: Uuid,
) -> Option<Arc<entities::user::Entity>> {
    loaders::cached::find_many_cached(cache, db_connection, Some(vec![user_id]))
        .await
        .pop()
}

pub async fn user_by_platform_role_id(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::user::Entity>,
    platform_role_id: Uuid,
) -> Vec<Arc<entities::user::Entity>> {
    let ids_to_load = loaders::cacheless::find_many_ids_related_associative::<
        entities::platform_role::Entity,
        entities::user::Entity,
        entities::user_role::Entity,
    >(db_connection, platform_role_id)
    .await;

    loaders::cached::find_many_cached(cache, db_connection, Some(ids_to_load)).await
}