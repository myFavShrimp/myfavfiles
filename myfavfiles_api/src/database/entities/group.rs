use mini_orm::{
    entity::{Identifiable, TableEntity},
    macros::iden,
    relation::OneToXRelation,
};
use uuid::Uuid;

use crate::database::entities;

iden! {
    Table => "group",
    Id => "id",
    Name => "name",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
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
        vec![Iden::Id, Iden::Name]
    }

    fn table() -> Iden {
        Iden::Table
    }
}

impl OneToXRelation<entities::group_member::Entity> for Entity {
    fn target_relation_id_column() -> <entities::group_member::Entity as TableEntity>::Iden {
        entities::group_member::Iden::GroupId
    }
}

impl OneToXRelation<entities::group_file_share::Entity> for Entity {
    fn target_relation_id_column() -> <entities::group_file_share::Entity as TableEntity>::Iden {
        entities::group_file_share::Iden::GroupId
    }
}

impl OneToXRelation<entities::group_role::Entity> for Entity {
    fn target_relation_id_column() -> <entities::group_role::Entity as TableEntity>::Iden {
        entities::group_role::Iden::GroupId
    }
}
