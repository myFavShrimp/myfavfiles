use uuid::Uuid;

use crate::database::{
    entities::{self, Identifiable},
    relation::OneToXRelation,
};

columns! {
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
        vec![Columns::Id, Columns::Name]
    }

    fn table() -> Columns {
        Columns::Table
    }
}

impl OneToXRelation<entities::group_member::Entity> for Entity {
    fn target_relation_id_column(
    ) -> <entities::group_member::Entity as entities::TableEntity>::ColumnsEnum {
        entities::group_member::Columns::GroupId
    }
}

impl OneToXRelation<entities::group_file_share::Entity> for Entity {
    fn target_relation_id_column(
    ) -> <entities::group_file_share::Entity as entities::TableEntity>::ColumnsEnum {
        entities::group_file_share::Columns::GroupId
    }
}

impl OneToXRelation<entities::group_role::Entity> for Entity {
    fn target_relation_id_column(
    ) -> <entities::group_role::Entity as entities::TableEntity>::ColumnsEnum {
        entities::group_role::Columns::GroupId
    }
}
