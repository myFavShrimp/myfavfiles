use uuid::Uuid;

use crate::database::{
    entities::{self, Identifiable},
    relation::{ManyToManyRelation, OneToXRelation},
};

columns! {
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

    fn id_column() -> Columns {
        Columns::Id
    }
}

impl super::TableEntity for Entity {
    type ColumnsEnum = Columns;

    fn all_columns() -> Vec<Columns> {
        vec![
            Columns::Id,
            Columns::Name,
            Columns::Password,
            Columns::IsAdmin,
        ]
    }

    fn table() -> Columns {
        Columns::Table
    }
}

impl OneToXRelation<entities::group_member::Entity> for Entity {
    fn target_relation_id_column(
    ) -> <entities::group_member::Entity as entities::TableEntity>::ColumnsEnum {
        entities::group_member::Columns::UserId
    }
}

impl OneToXRelation<entities::group_file_share::Entity> for Entity {
    fn target_relation_id_column(
    ) -> <entities::group_file_share::Entity as entities::TableEntity>::ColumnsEnum {
        entities::group_file_share::Columns::UserId
    }
}

impl OneToXRelation<entities::user_file_share::Entity> for Entity {
    fn target_relation_id_column(
    ) -> <entities::user_file_share::Entity as entities::TableEntity>::ColumnsEnum {
        entities::user_file_share::Columns::UserId
    }
}

impl ManyToManyRelation<entities::platform_role::Entity, entities::user_role::Entity> for Entity {
    fn own_relation_id_column(
    ) -> <entities::user_role::Entity as entities::TableEntity>::ColumnsEnum {
        entities::user_role::Columns::UserId
    }

    fn other_entity_id(entity: entities::user_role::Entity) -> Uuid {
        entity.user_id
    }
}
