use std::sync::Arc;

use uuid::Uuid;

use crate::database::{
    actions::build_insert_query, cache::Cache, entities, loaders, PoolConnection,
};

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

pub async fn groups_by_ids(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group::Entity>,
    group_ids: Option<Vec<Uuid>>,
) -> Result<Vec<Arc<entities::group::Entity>>, DataError> {
    Ok(loaders::cached::find_many_cached(cache, db_connection, group_ids).await?)
}

pub async fn create_group(
    db_connection: &mut PoolConnection,
    cache: Cache<entities::group::Entity>,
    group_name: String,
    admin_user: Uuid,
) -> Result<Arc<entities::group::Entity>, DataError> {
    let (sql, values) = build_insert_query(
        entities::group::Iden::Table,
        vec![entities::group::Iden::Name],
        vec![group_name.into()],
    )?;

    let query = sqlx::query_as_with::<_, entities::group::Entity, _>(&sql, values);
    Ok(Arc::new(query.fetch_one(db_connection).await?))
}
