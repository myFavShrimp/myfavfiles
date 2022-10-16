use uuid::Uuid;

use crate::database::{
    entities::{self, Identifiable},
    relation::ManyToManyRelation,
};

columns! {
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
            Columns::IsAdmin,
        ]
    }

    fn table() -> Columns {
        Columns::Table
    }
}

impl ManyToManyRelation<entities::group_role::Entity, entities::group_member_role::Entity>
    for Entity
{
    fn own_relation_id_column(
    ) -> <entities::group_member_role::Entity as entities::TableEntity>::ColumnsEnum {
        entities::group_member_role::Columns::GroupMemberId
    }

    fn other_entity_id(entity: entities::group_member_role::Entity) -> Uuid {
        entity.group_member_id
    }
}
