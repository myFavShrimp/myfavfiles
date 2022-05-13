use uuid::Uuid;

pub mod group;
pub mod group_member;
pub mod user;
pub mod platform_role;

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct IdEntity {
    pub id: Uuid,
}

pub trait TableEntity {
    type ColumnsEnum;

    fn all_columns() -> Vec<Self::ColumnsEnum>;

    fn id_column() -> Self::ColumnsEnum;

    fn table() -> Self::ColumnsEnum;
}

pub trait RelationColumn<OtherColumnsEnum> {
    fn relation_id_column() -> Self;
}
