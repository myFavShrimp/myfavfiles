use uuid::Uuid;

pub mod group;
pub mod group_file_share;
pub mod group_member;
pub mod group_member_role;
pub mod group_role;
pub mod platform_role;
pub mod user;
pub mod user_file_share;
pub mod user_role;

pub trait Identifiable: TableEntity {
    fn id(&self) -> Uuid;

    fn id_column() -> Self::ColumnsEnum;
}

pub trait TableEntity {
    type ColumnsEnum;

    fn all_columns() -> Vec<Self::ColumnsEnum>;

    fn table() -> Self::ColumnsEnum;
}

pub mod id_entity {
    use uuid::Uuid;

    use super::{Identifiable, TableEntity};

    columns! {
        Id => "id",
    }

    #[derive(sqlx::FromRow, Debug, Clone)]
    #[allow(dead_code)]
    pub struct IdEntity {
        pub id: Uuid,
    }

    impl TableEntity for IdEntity {
        type ColumnsEnum = Columns;

        fn all_columns() -> Vec<Self::ColumnsEnum> {
            vec![Columns::Id]
        }

        fn table() -> Self::ColumnsEnum {
            panic!("IdEntity does not have a table")
        }
    }

    impl Identifiable for IdEntity {
        fn id(&self) -> Uuid {
            self.id
        }

        fn id_column() -> Self::ColumnsEnum {
            Columns::Id
        }
    }

    pub fn into_vec_uuid(items: impl Iterator<Item = IdEntity>) -> Vec<Uuid> {
        items.fold(Vec::new(), |mut acc, item| {
            acc.push(item.id);

            acc
        })
    }
}
