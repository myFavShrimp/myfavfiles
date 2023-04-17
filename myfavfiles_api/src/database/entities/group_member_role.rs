use mini_orm::{entity::TableEntity, macros::iden};
use uuid::Uuid;

iden! {
    Table => "group_member_role",
    GroupMemberId => "group_member_id",
    GroupRoleId => "group_role_id",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub group_member_id: Uuid,
    pub group_role_id: Uuid,
}

impl TableEntity for Entity {
    type Iden = Iden;

    fn all_columns() -> Vec<Iden> {
        vec![Iden::GroupMemberId, Iden::GroupRoleId]
    }

    fn table() -> Iden {
        Iden::Table
    }
}
