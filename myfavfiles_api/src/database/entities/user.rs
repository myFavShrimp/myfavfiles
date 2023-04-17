use mini_orm::{
    entity::{Identifiable, TableEntity},
    macros::iden,
    relation::{ManyToManyRelation, OneToXRelation},
};
use uuid::Uuid;

use crate::database::entities;

iden! {
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
        vec![Iden::Id, Iden::Name, Iden::Password, Iden::IsAdmin]
    }

    fn table() -> Iden {
        Iden::Table
    }
}

impl OneToXRelation<entities::group_member::Entity> for Entity {
    fn target_relation_id_column() -> <entities::group_member::Entity as TableEntity>::Iden {
        entities::group_member::Iden::UserId
    }
}

impl OneToXRelation<entities::group_file_share::Entity> for Entity {
    fn target_relation_id_column() -> <entities::group_file_share::Entity as TableEntity>::Iden {
        entities::group_file_share::Iden::UserId
    }
}

impl OneToXRelation<entities::user_file_share::Entity> for Entity {
    fn target_relation_id_column() -> <entities::user_file_share::Entity as TableEntity>::Iden {
        entities::user_file_share::Iden::UserId
    }
}

impl ManyToManyRelation<entities::platform_role::Entity, entities::user_role::Entity> for Entity {
    fn own_relation_id_column() -> <entities::user_role::Entity as TableEntity>::Iden {
        entities::user_role::Iden::UserId
    }

    fn other_entity_id(entity: entities::user_role::Entity) -> Uuid {
        entity.platform_role_id
    }
}
