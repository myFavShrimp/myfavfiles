use uuid::Uuid;

use crate::{database::entities, database::loaders::Identifiable};

#[allow(dead_code)]
pub enum Columns {
    Table,
    Id,
    UserId,
    GroupId,
    IsAdmin,
}

impl_iden! {
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

impl Identifiable for Entity {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl super::TableEntity<Columns> for Entity {
    fn all_columns() -> Vec<Columns> {
        vec![
            Columns::Id,
            Columns::UserId,
            Columns::GroupId,
            Columns::IsAdmin,
        ]
    }

    fn id_column() -> Columns {
        Columns::Id
    }

    fn table() -> Columns {
        Columns::Table
    }
}

impl super::RelationColumn<entities::user::Columns, Columns> for Columns {
    fn get_relation_id_column() -> Columns {
        Columns::UserId
    }
}

impl super::RelationColumn<entities::group::Columns, Columns> for Columns {
    fn get_relation_id_column() -> Columns {
        Columns::GroupId
    }
}
