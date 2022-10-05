use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::database::{
    entities::{self, Identifiable},
    relation::OneToXRelation,
};

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

    fn id_column() -> Columns {
        Columns::Id
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
