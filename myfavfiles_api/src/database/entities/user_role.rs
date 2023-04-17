use mini_orm::{entity::TableEntity, macros::iden};
use uuid::Uuid;

iden! {
    Table => "user_role",
    UserId => "user_id",
    PlatformRoleId => "platform_role_id",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub user_id: Uuid,
    pub platform_role_id: Uuid,
}

impl TableEntity for Entity {
    type Iden = Iden;

    fn all_columns() -> Vec<Iden> {
        vec![Iden::UserId, Iden::PlatformRoleId]
    }

    fn table() -> Iden {
        Iden::Table
    }
}
