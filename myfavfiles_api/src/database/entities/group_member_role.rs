use uuid::Uuid;

use crate::database::entities;

use super::AssociationEntity;

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

impl AssociationEntity<entities::group_role::Columns> for Entity {
    fn id(&self) -> Uuid {
        self.group_role_id
    }
}

impl AssociationEntity<entities::group_member::Columns> for Entity {
    fn id(&self) -> Uuid {
        self.group_member_id
    }
}

impl super::RelationColumn<entities::group_member::Columns> for Columns {
    fn relation_id_column() -> Self {
        Columns::GroupMemberId
    }
}

impl super::RelationColumn<entities::group_role::Columns> for Columns {
    fn relation_id_column() -> Self {
        Columns::GroupRoleId
    }
}
