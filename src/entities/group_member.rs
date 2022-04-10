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
    UserId,
    GroupId,
    IsAdmin,
}

crate::entities::macros::impl_iden! {
    Columns,
    Table => "group_member",
    Id => "id",
    UserId => "user_id",
    GroupId => "group_id",
    IsAdmin => "is_admin",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub is_admin: bool,
}

#[graphql_object(Context = Context, name = "Group")]
impl Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn user_id(&self) -> Uuid {
        self.id
    }

    async fn group_id(&self) -> Uuid {
        self.id
    }

    async fn is_admin(&self) -> bool {
        self.is_admin
    }
}

impl GetId for Entity {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
