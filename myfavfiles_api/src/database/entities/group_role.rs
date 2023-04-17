use mini_orm::{
    entity::{Identifiable, TableEntity},
    macros::iden,
    relation::ManyToManyRelation,
};
use sqlx::postgres::PgHasArrayType;
use uuid::Uuid;

use crate::database::entities;

iden! {
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
        vec![Iden::Id, Iden::Name, Iden::GroupId, Iden::Permissions]
    }

    fn table() -> Iden {
        Iden::Table
    }
}

#[derive(Copy, Clone, Debug, sqlx::Type, async_graphql::Enum, Eq, PartialEq)]
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

impl ManyToManyRelation<entities::group_member::Entity, entities::group_member_role::Entity>
    for Entity
{
    fn own_relation_id_column() -> <entities::group_member_role::Entity as TableEntity>::Iden {
        entities::group_member_role::Iden::GroupRoleId
    }

    fn other_entity_id(entity: entities::group_member_role::Entity) -> Uuid {
        entity.group_member_id
    }
}
