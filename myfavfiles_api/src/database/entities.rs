use sea_query::Iden;
use uuid::Uuid;

pub mod group;
pub mod group_member;
pub mod user;

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct IdEntity {
    pub id: Uuid,
}

pub trait TableEntity<ColumnType>
where
    ColumnType: sea_query::Iden,
{
    fn all_columns() -> Vec<ColumnType>;

    fn id_column() -> ColumnType;

    fn table() -> ColumnType;
}

pub trait RelationColumn<OtherColumnType, ColumnType>
where
    OtherColumnType: Iden,
    ColumnType: Iden,
{
    fn get_relation_id_column() -> ColumnType;
}
