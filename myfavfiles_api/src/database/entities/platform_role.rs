use mini_orm::{
    entity::{Identifiable, TableEntity},
    macros::iden,
    relation::ManyToManyRelation,
};
use sqlx::postgres::PgHasArrayType;
use uuid::Uuid;

use crate::database::entities;

iden! {
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
    type IdType = Uuid;

    fn id(&self) -> Uuid {
        self.id
    }

    fn id_column() -> Iden {
        Iden::Id
    }
}

impl TableEntity for Entity {
    type Iden = Iden;

    fn all_columns() -> Vec<Iden> {
        vec![Iden::Id, Iden::Name, Iden::Permissions]
    }

    fn table() -> Iden {
        Iden::Table
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
    fn own_relation_id_column() -> <entities::user_role::Entity as TableEntity>::Iden {
        entities::user_role::Iden::PlatformRoleId
    }

    fn other_entity_id(entity: entities::user_role::Entity) -> Uuid {
        entity.user_id
    }
}
