use uuid::Uuid;

columns! {
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

impl super::TableEntity for Entity {
    type ColumnsEnum = Columns;

    fn all_columns() -> Vec<Columns> {
        vec![Columns::GroupMemberId, Columns::GroupRoleId]
    }

    fn table() -> Columns {
        Columns::Table
    }
}
