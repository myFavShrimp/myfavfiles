use mini_orm::{
    entity::{Identifiable, TableEntity},
    macros::iden,
    relation::ManyToManyRelation,
};
use uuid::Uuid;

use crate::database::entities;

iden! {
    Table => "group_member",
    Id => "id",
    UserId => "user_id",
    GroupId => "group_id",
    IsAdmin => "is_admin",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub is_admin: bool,
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
        vec![Iden::Id, Iden::UserId, Iden::GroupId, Iden::IsAdmin]
    }

    fn table() -> Iden {
        Iden::Table
    }
}

impl ManyToManyRelation<entities::group_role::Entity, entities::group_member_role::Entity>
    for Entity
{
    fn own_relation_id_column() -> <entities::group_member_role::Entity as TableEntity>::Iden {
        entities::group_member_role::Iden::GroupMemberId
    }

    fn other_entity_id(entity: entities::group_member_role::Entity) -> Uuid {
        entity.group_role_id
    }
}
