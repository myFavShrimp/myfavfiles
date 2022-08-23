use std::{fmt::Debug, ops::DerefMut, sync::Arc};

use sea_query::{Iden, Values};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use crate::handlers::graphql::authenticated::Context;

use crate::database::driver::bind_query_as;

use super::{
    actions::build_select_query,
    cache::HasCache,
    entities::{AssociationEntity, IdColumn, IdEntity, RelationColumn, TableEntity},
};

pub mod group;
pub mod group_file_share;
pub mod group_member;
pub mod group_role;
pub mod platform_role;
pub mod user;
pub mod user_file_share;

#[derive(Default)]
pub struct Loaders {
    pub user: user::UserLoader,
    pub group: group::GroupLoader,
    pub group_member: group_member::GroupMemberLoader,
    pub platform_role: platform_role::PlatformRoleLoader,
    pub group_role: group_role::GroupRoleLoader,
    pub group_file_share: group_file_share::GroupFileShare,
    pub user_file_share: user_file_share::UserFileShare,
}

#[async_trait::async_trait]
pub trait Loader: HasCache<Self::LoadableEntity>
where
    Self: std::marker::Send + Sync,
    Self::LoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity
        + IdColumn
        + Debug,
    <Self::LoadableEntity as TableEntity>::ColumnsEnum: Iden + Send + 'static,
    sea_query::Value: From<Uuid>,
{
    type LoadableEntity;

    async fn load_many(
        &mut self,
        ctx: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> Vec<Arc<Self::LoadableEntity>> {
        let cached_ids = self.all_cached().await;
        let mut results = self.get_all(&cached_ids).await;

        let ids_to_load = ids.map(|ids| {
            ids.into_iter()
                .filter(|id| cached_ids.contains(id))
                .collect()
        });

        let columns = Self::LoadableEntity::all_columns();
        let id_column = Self::LoadableEntity::id_column();
        let table = Self::LoadableEntity::table();
        let (sql, values) = build_select_query(columns, table, id_column, ids_to_load);

        let mut mutex = ctx.database_connection.lock().await;
        let conn = mutex.deref_mut();
        let mut _cache = self.cache();
        let mut cache = _cache.lock().await;

        Self::query(conn, sql, values)
            .await
            .iter()
            .for_each(|item| {
                let arc_item = Arc::new(item.clone());
                cache.insert(arc_item.id(), arc_item.clone());
                results.push(arc_item);
            });

        results
    }

    async fn query(
        conn: &mut super::PoolConnection,
        sql: String,
        values: Values,
    ) -> Vec<Self::LoadableEntity> {
        let query = bind_query_as(sqlx::query_as::<_, Self::LoadableEntity>(&sql), &values);
        if let Ok(rows) = query.fetch_all(conn).await {
            rows.iter().fold(Vec::new(), |mut acc, item| {
                acc.push(item.clone());

                acc
            })
        } else {
            Vec::new()
        }
    }
}

pub trait Identifiable {
    fn id(&self) -> Uuid;
}

#[async_trait::async_trait]
pub trait LoadableRelationOneToMany<OtherColumnsEnum>: Loader
where
    <<Self as Loader>::LoadableEntity as TableEntity>::ColumnsEnum:
        Iden + Send + 'static + RelationColumn<OtherColumnsEnum>,
    OtherColumnsEnum: Iden + Send + 'static,
{
    async fn query_ids(ctx: &Context, sql: String, values: Values) -> Vec<Uuid> {
        let mut mutex = ctx.database_connection.lock().await;
        let conn = mutex.deref_mut();
        let query = bind_query_as(sqlx::query_as::<_, IdEntity>(&sql), &values);
        if let Ok(rows) = query.fetch_all(conn).await {
            rows.iter().fold(Vec::new(), |mut acc, item| {
                acc.push(item.id);

                acc
            })
        } else {
            Vec::new()
        }
    }

    async fn load_many_related(
        &mut self,
        ctx: &Context,
        ids: Vec<Uuid>,
    ) -> Vec<Arc<<Self as Loader>::LoadableEntity>> {
        let id_column = <Self as Loader>::LoadableEntity::id_column();
        let table = <Self as Loader>::LoadableEntity::table();

        let (sql, values) = build_select_query(
            vec![id_column],
            table,
            <<Self as Loader>::LoadableEntity as TableEntity>::ColumnsEnum::relation_id_column(),
            Some(ids),
        );
        let relational_ids = Self::query_ids(ctx, sql, values).await;

        self.load_many(ctx, Some(relational_ids)).await
    }
}

#[async_trait::async_trait]
pub trait LoadableRelationManyToMany<OtherColumnsEnum>: Loader
where
    <<Self as Loader>::LoadableEntity as TableEntity>::ColumnsEnum: Iden + Send + 'static,
    OtherColumnsEnum: Iden + Send + 'static,
    Self::AssociationEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Sync
        + TableEntity
        + AssociationEntity<<<Self as Loader>::LoadableEntity as TableEntity>::ColumnsEnum>
        + Debug,
    <Self::AssociationEntity as TableEntity>::ColumnsEnum: Iden
        + Send
        + 'static
        + RelationColumn<OtherColumnsEnum>
        + RelationColumn<<<Self as Loader>::LoadableEntity as TableEntity>::ColumnsEnum>
        + Debug,
{
    type AssociationEntity;

    fn associated_id(entity: Self::AssociationEntity) -> Uuid;

    async fn query_ids(ctx: &Context, sql: String, values: Values) -> Vec<Uuid> {
        let mut mutex = ctx.database_connection.lock().await;
        let conn = mutex.deref_mut();
        let query = bind_query_as(sqlx::query_as::<_, Self::AssociationEntity>(&sql), &values);

        if let Ok(rows) = query.fetch_all(conn).await {
            rows.iter().fold(Vec::new(), |mut acc, item| {
                acc.push(item.id());

                acc
            })
        } else {
            Vec::new()
        }
    }

    async fn load_many_related(
        &mut self,
        ctx: &Context,
        ids: Vec<Uuid>,
    ) -> Vec<Arc<<Self as Loader>::LoadableEntity>> {
        let id_column = <<Self::AssociationEntity as TableEntity>::ColumnsEnum as RelationColumn<
            OtherColumnsEnum,
        >>::relation_id_column();
        let table = <Self::AssociationEntity as TableEntity>::table();

        let (sql, values) = build_select_query(
            <Self::AssociationEntity as TableEntity>::all_columns(),
            table,
            id_column,
            Some(ids),
        );

        let relational_ids = Self::query_ids(ctx, sql, values).await;

        self.load_many(ctx, Some(relational_ids)).await
    }
}
