use chrono::NaiveDateTime;
use mini_orm::{
    entity::{Identifiable, TableEntity},
    macros::iden,
};
use uuid::Uuid;

iden! {
    Table => "group_file_share",
    Id => "id",
    UserId => "user_id",
    GroupId => "group_id",
    Expiration => "expiration",
}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub expiration: NaiveDateTime,
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
        vec![Iden::Id, Iden::UserId, Iden::GroupId, Iden::Expiration]
    }

    fn table() -> Iden {
        Iden::Table
    }
}
