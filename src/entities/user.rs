use uuid::Uuid;

use crate::handlers::graphql::loaders::GetId;

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

#[derive(sqlx::FromRow, Debug, Clone, juniper::GraphQLObject)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub is_admin: bool,
}

impl GetId for Entity {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
