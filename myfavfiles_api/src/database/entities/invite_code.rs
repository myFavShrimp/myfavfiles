use uuid::Uuid;

use crate::database::loaders::Identifiable;

columns! {
    Table => "invite_code",
    Id => "id",
    Expiration => "expiration",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub expiration: String,
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
