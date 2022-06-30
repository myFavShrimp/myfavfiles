use juniper::GraphQLEnum;
use sqlx::postgres::PgHasArrayType;
use uuid::Uuid;

use crate::database::loaders::Identifiable;
use crate::database::entities;

columns! {
    Table => "group_role",
    Id => "id",
    Name => "name",
    GroupId => "group_id",
    Permissions => "permissions",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub group_id: Uuid,
    pub permissions: Option<Vec<GroupRolePermission>>,
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
            Columns::Name,
            Columns::GroupId,
            Columns::Permissions,
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

#[derive(Copy, Clone, Debug, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "group_permissions_enum", rename_all = "snake_case")]

pub enum GroupRolePermission {
    CreateInviteCode,
    KickUser,
    Administrator,
    UploadFiles,
    DeleteFiles,
    ManageRoles,
}

impl PgHasArrayType for GroupRolePermission {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_group_permissions_enum")
    }
}

impl super::RelationColumn<entities::group::Columns> for Columns {
    fn relation_id_column() -> Columns {
        Columns::GroupId
    }
}
