use uuid::Uuid;

use crate::database::loaders::Identifiable;

#[allow(dead_code)]
pub enum Columns {
    Table,
    Id,
    Name,
}

crate::database::entities::macros::impl_iden! {
    Columns,
    Table => "group",
    Id => "id",
    Name => "name",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
}

impl Identifiable for Entity {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl super::TableEntity<Columns> for Entity {
    fn all_columns() -> Vec<Columns> {
        vec![Columns::Id, Columns::Name]
    }

    fn id_column() -> Columns {
        Columns::Id
    }

    fn table() -> Columns {
        Columns::Table
    }
}
