use std::{sync::Arc, collections::HashMap};

use juniper::futures::lock::Mutex;
use sea_query::{Values, Query, Expr, PostgresQueryBuilder, Iden};
use uuid::Uuid;

use super::Context;
sea_query::sea_query_driver_postgres!();

pub mod user;
pub mod group;

#[derive(Default)]
pub struct Loaders {
    pub user: user::UserLoader,
    pub group: group::GroupLoader,
}

pub type Cache<I, E> = Arc<Mutex<HashMap<I, E>>>;

#[async_trait::async_trait]
pub trait Loadable {
    type IdentifierType;
    type LoadableType;

    async fn load_many(&mut self, ctx: &Context, ids: Option<Vec<Self::IdentifierType>>) -> Vec<Arc<Self::LoadableType>>;

    fn get_cache(&mut self) -> Cache<Self::IdentifierType, Arc<Self::LoadableType>>;
}

pub fn build_select_query<E>(columns: Vec<E>, table: E, id_column: E, ids_to_load: Option<Vec<Uuid>>) -> (String, Values)
where
    E: Iden + 'static
{
    match ids_to_load {
        Some(ids_to_load) => Query::select()
            .columns(columns)
            .from(table)
            .and_where(Expr::col(id_column).is_in(ids_to_load))
            .build(PostgresQueryBuilder),
        None => Query::select()
            .columns(columns)
            .from(table)
            .build(PostgresQueryBuilder),
    }
}
