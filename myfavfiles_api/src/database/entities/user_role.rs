use uuid::Uuid;

use crate::database::entities;

use super::AssociationEntity;

columns! {
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

impl super::TableEntity for Entity {
    type ColumnsEnum = Columns;

    fn all_columns() -> Vec<Columns> {
        vec![Columns::UserId, Columns::PlatformRoleId]
    }

    fn table() -> Columns {
        Columns::Table
    }
}

impl AssociationEntity<entities::platform_role::Columns> for Entity {
    fn id(&self) -> Uuid {
        self.platform_role_id
    }
}

impl super::RelationColumn<entities::user::Columns> for Columns {
    fn relation_id_column() -> Self {
        Columns::UserId
    }
}

impl super::RelationColumn<entities::platform_role::Columns> for Columns {
    fn relation_id_column() -> Self {
        Columns::PlatformRoleId
    }
}
