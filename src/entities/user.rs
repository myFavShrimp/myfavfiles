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
    Password,
    IsAdmin,
}

crate::entities::macros::impl_iden! {
    Columns,
    Table => "user",
    Id => "id",
    Name => "name",
    Password => "password",
    IsAdmin => "is_admin",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub is_admin: bool,
}

#[graphql_object(Context = Context, name = "User")]
impl Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn is_admin(&self) -> bool {
        self.is_admin
    }

    async fn group(context: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<entities::group::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.group.load_many(context, ids).await
    }
}

impl GetId for Entity {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
