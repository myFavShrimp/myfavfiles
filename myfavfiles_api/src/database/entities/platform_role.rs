use sqlx::postgres::PgHasArrayType;
use uuid::Uuid;

use crate::database::{
    entities::{self, Identifiable},
    relation::ManyToManyRelation,
};

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
    pub permissions: Vec<PlatformRolePermission>,
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
        vec![Columns::Id, Columns::Name, Columns::Permissions]
    }

    fn table() -> Columns {
        Columns::Table
    }
}

#[derive(Copy, Clone, Debug, sqlx::Type, async_graphql::Enum, Eq, PartialEq)]
#[sqlx(type_name = "platform_permissions_enum", rename_all = "snake_case")]
pub enum PlatformRolePermission {
    CreateInviteCode,  // -- invite users to platform
    Administrator,     // -- everything
    HasPrivateStorage, // -- crud user files
    ManageRoles,       // -- crud roles + user assignement, only permissions of self
    SeeUsers,          // --list users
    BanUsers,
    SeeGroups,    // -- list groups, no access
    ManageGroups, // ? -- crud & file access
    CreateGroups, // -- creator will become admin of group
}

impl PgHasArrayType for PlatformRolePermission {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_platform_permissions_enum")
    }
}

impl ManyToManyRelation<entities::user::Entity, entities::user_role::Entity> for Entity {
    fn own_relation_id_column(
    ) -> <entities::user_role::Entity as entities::TableEntity>::ColumnsEnum {
        entities::user_role::Columns::PlatformRoleId
    }

    fn other_entity_id(entity: entities::user_role::Entity) -> Uuid {
        entity.user_id
    }
}
