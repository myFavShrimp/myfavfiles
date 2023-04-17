pub mod group;
pub mod group_file_share;
pub mod group_member;
pub mod group_member_role;
pub mod group_role;
pub mod platform_role;
pub mod user;
pub mod user_file_share;
pub mod user_role;

pub mod id_entity {
    use mini_orm::{
        entity::{Identifiable, TableEntity},
        macros::iden,
    };

    iden! {
        Id => "id",
    }

    #[derive(Debug, Clone, sqlx::FromRow)]
    pub struct IdEntity<I>
    where
        I: Copy + Send + Sync,
    {
        pub id: I,
    }

    impl<I> TableEntity for IdEntity<I>
    where
        I: Copy + Send + Sync,
    {
        type Iden = Iden;

        fn all_columns() -> Vec<Self::Iden> {
            vec![Iden::Id]
        }

        fn table() -> Self::Iden {
            panic!("IdEntity does not have a table")
        }
    }

    impl<I> Identifiable for IdEntity<I>
    where
        I: Copy + Send + Sync,
    {
        type IdType = I;

        fn id(&self) -> I {
            self.id
        }

        fn id_column() -> Self::Iden {
            Iden::Id
        }
    }
}
