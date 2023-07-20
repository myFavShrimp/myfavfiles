use std::sync::Arc;

use sqlx::Acquire;
use uuid::Uuid;

use crate::database::{
    actions::build_insert_query, cache::Cache, entities, loaders, PoolConnection,
};

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

pub async fn create_group_role(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group_role::Entity>,
    group_role_name: String,
    group_id: Uuid,
) -> Result<Arc<entities::group_role::Entity>, DataError> {
    let cache_lock = cache.cache_map();
    let mut cache_map = cache_lock.lock().await;

    let created_group_role = {
        let (sql, values) = build_insert_query(
            entities::group_role::Iden::Table,
            vec![
                entities::group_role::Iden::Name,
                entities::group_role::Iden::GroupId,
            ],
            vec![group_role_name.into(), group_id.into()],
        )?;

        let query = sqlx::query_as_with::<_, entities::group_role::Entity, _>(&sql, values);
        let conn = db_connection.acquire().await.unwrap();
        query.fetch_one(conn).await?
    };

    let created_group_role_arc = Arc::new(created_group_role);
    cache_map.insert(created_group_role_arc.id, created_group_role_arc.clone());
    Ok(created_group_role_arc)
}
