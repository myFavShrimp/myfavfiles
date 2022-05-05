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

mod macros {
    #[macro_export]
    macro_rules! impl_iden {
        ($enum: ty, $($x: ident => $y: literal,)+) => {
            use sea_query::Iden;

            impl Iden for $enum {
                fn unquoted(&self, s: &mut dyn std::fmt::Write) {
                    write!(
                        s,
                        "{}",
                        match self {
                            $(Self::$x => $y,)+
                        }
                    )
                    .unwrap();
                }
            }
        };
    }

    pub use impl_iden;
}

pub trait TableEntity<ColumnType>
where
    ColumnType: sea_query::Iden,
{
    fn all_columns() -> Vec<ColumnType>;

    fn id_column() -> ColumnType;

    fn table() -> ColumnType;
}

pub trait RelationOneToOne<OtherEntity, OtherColumnType, ColumnType>
where
    OtherEntity: TableEntity<OtherColumnType>,
    Self: TableEntity<ColumnType>,
    OtherColumnType: Iden,
    ColumnType: Iden,
{
    fn get_relation_id_column() -> ColumnType;
}
