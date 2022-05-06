use uuid::Uuid;

use crate::{database::entities, database::loaders::Identifiable};

#[allow(dead_code)]
pub enum Columns {
    Table,
    Id,
    Name,
    Password,
    IsAdmin,
}

entities::macros::impl_iden! {
    Columns,
    Table => "user",
    Id => "id",
    Name => "name",
    Password => "password",
    IsAdmin => "is_admin",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub password: String,
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
            Columns::Name,
            Columns::Password,
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
