use juniper::GraphQLEnum;
use sqlx::postgres::PgHasArrayType;
use uuid::Uuid;

use crate::database::loaders::Identifiable;

columns! {
    Table => "platform_role",
    Id => "id",
    Name => "name",
    Permissions => "permissions",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub permissions: Vec<Permission>,
}

impl Identifiable for Entity {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl super::TableEntity for Entity {
    type ColumnsEnum = Columns;

    fn all_columns() -> Vec<Columns> {
        vec![Columns::Id, Columns::Name, Columns::Permissions]
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

#[derive(Copy, Clone, Debug, sqlx::Type, GraphQLEnum)]
#[sqlx(
    type_name = "platform_permissions_enum", 
    rename_all = "snake_case"
)]

pub enum Permission {
    CreateInviteCode,
    BanUser,
    Administrator,
    ManageGroups,
    CreateGroups,
    UploadFiles,
    DeleteFiles,
    ManageRoles,
}

impl PgHasArrayType for Permission {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_platform_permissions_enum")
    }
}
