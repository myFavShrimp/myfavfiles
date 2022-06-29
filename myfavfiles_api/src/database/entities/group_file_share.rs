use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{database::entities, database::loaders::Identifiable};

columns! {
    Table => "group_file_share",
    Id => "id",
    UserId => "user_id",
    GroupId => "group_id",
    Expiration => "expiration",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub expiration: NaiveDateTime,
}

impl Identifiable for Entity {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl super::TableEntity for Entity {
    type ColumnsEnum = Columns;

    fn all_columns() -> Vec<Columns> {
        vec![
            Columns::Id,
            Columns::UserId,
            Columns::GroupId,
            Columns::Expiration,
        ]
    }

    fn table() -> Columns {
        Columns::Table
    }
}

impl super::IdColumn for Entity {
    fn id_column() -> Columns {
        Columns::Id
    }
}

impl super::RelationColumn<entities::user::Columns> for Columns {
    fn relation_id_column() -> Columns {
        Columns::UserId
    }
}

impl super::RelationColumn<entities::group::Columns> for Columns {
    fn relation_id_column() -> Columns {
        Columns::GroupId
    }
}
