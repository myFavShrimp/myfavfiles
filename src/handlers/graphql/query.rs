use std::sync::Arc;

use uuid::Uuid;

use crate::{entities, handlers::graphql::loaders::{Loaders, Loadable}};

use super::Context;


pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    fn hello(context: &Context) -> String {
        context.app_state.as_ref().config.database_url.clone()
    }

    async fn users(context: &Context) -> Vec<Arc<entities::user::Entity>> {
        let mut loaders = Loaders::default();

        loaders.user.load_many(context, &[Uuid::nil()]).await
    }
}
