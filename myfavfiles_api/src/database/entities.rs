use sea_query::Iden;
use uuid::Uuid;

pub mod group;
pub mod group_member;
pub mod platform_role;
pub mod user;
pub mod user_role;

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct IdEntity {
    pub id: Uuid,
}

pub trait TableEntity {
    type ColumnsEnum;

    fn all_columns() -> Vec<Self::ColumnsEnum>;

    fn table() -> Self::ColumnsEnum;
}

pub trait IdColumn: TableEntity {
    fn id_column() -> Self::ColumnsEnum;
}

pub trait AssociationEntity<OtherColumnsEnum> {
    fn id(&self) -> Uuid;
}

pub trait RelationColumn<OtherColumnsEnum>: Iden {
    fn relation_id_column() -> Self;
}
