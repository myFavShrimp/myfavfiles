use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use crate::{
    entities,
    handlers::graphql::{
        loaders::{GetId, Loadable},
        Context,
    },
};

#[allow(dead_code)]
pub enum Columns {
    Table,
    Id,
    Name,
}

crate::entities::macros::impl_iden! {
    Columns,
    Table => "group",
    Id => "id",
    Name => "name",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
}

#[graphql_object(Context = Context, name = "Group")]
impl Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn user(context: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.user.load_many(context, ids).await
    }
}

impl GetId for Entity {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
