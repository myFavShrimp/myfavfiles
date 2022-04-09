use std::sync::Arc;

use super::Context;
sea_query::sea_query_driver_postgres!();

pub mod user;

#[derive(Default)]
pub struct Loaders {
    pub user: user::UserLoader,
}

#[async_trait::async_trait]
pub trait Loadable {
    type IdentifierType;
    type LoadableType;

    async fn load_many(&mut self, ctx: &Context, ids: &[Self::IdentifierType]) -> Vec<Arc<Self::LoadableType>>;

    // async fn load_one(&mut self, ctx: &Context, id: Self::IdentifierType) -> Arc<Self::LoadableType> {
    //     let id_slice = &[id];

    //     self.load_many(ctx, id_slice).await
    //         .pop().unwrap()
    // }
}
