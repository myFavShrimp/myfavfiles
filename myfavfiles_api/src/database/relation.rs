use sea_query::Iden;
use uuid::Uuid;

use super::entities::{Identifiable, TableEntity};

pub mod implementation;

pub trait OneToXRelation<B>
where
    B: TableEntity + Identifiable,
    B::ColumnsEnum: Iden,
{
    fn target_relation_id_column() -> B::ColumnsEnum;
}

pub trait ManyToManyRelation<B, R>
where
    B: TableEntity + Identifiable,
    R: TableEntity,
{
    fn own_relation_id_column() -> R::ColumnsEnum;

    fn other_entity_id(entity: R) -> Uuid;
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::database::{
        entities::{Identifiable, TableEntity},
        relation::OneToXRelation,
    };

    use super::ManyToManyRelation;

    mod a {
        columns! {
            Table => "table",
            Id => "id",
            BId => "b_id",
        }

        pub struct Entity {
            id: super::Uuid,
        }

        impl super::TableEntity for Entity {
            type ColumnsEnum = Columns;

            fn all_columns() -> Vec<Self::ColumnsEnum> {
                vec![Columns::Id, Columns::BId]
            }

            fn table() -> Self::ColumnsEnum {
                Columns::Table
            }
        }

        impl super::Identifiable for Entity {
            fn id(&self) -> super::Uuid {
                self.id
            }

            fn id_column() -> Self::ColumnsEnum {
                Columns::Id
            }
        }
    }

    mod b {
        columns! {
            Table => "table",
            Id => "id",
            AId => "a_id",
        }

        pub struct Entity {
            id: super::Uuid,
        }

        impl super::TableEntity for Entity {
            type ColumnsEnum = Columns;

            fn all_columns() -> Vec<Self::ColumnsEnum> {
                vec![Columns::Id, Columns::AId]
            }

            fn table() -> Self::ColumnsEnum {
                Columns::Table
            }
        }

        impl super::Identifiable for Entity {
            fn id(&self) -> super::Uuid {
                self.id
            }

            fn id_column() -> Self::ColumnsEnum {
                Columns::Id
            }
        }
    }

    mod r {
        columns! {
            Table => "table",
            AId => "a_id",
            BId => "b_id",
        }

        pub struct Entity {
            pub a_id: super::Uuid,
            pub b_id: super::Uuid,
        }

        impl super::TableEntity for Entity {
            type ColumnsEnum = Columns;

            fn all_columns() -> Vec<Self::ColumnsEnum> {
                vec![Columns::AId, Columns::BId]
            }

            fn table() -> Self::ColumnsEnum {
                Columns::Table
            }
        }
    }

    impl OneToXRelation<b::Entity> for a::Entity {
        fn target_relation_id_column() -> <b::Entity as TableEntity>::ColumnsEnum {
            b::Columns::AId
        }
    }
    impl OneToXRelation<a::Entity> for b::Entity {
        fn target_relation_id_column() -> <a::Entity as TableEntity>::ColumnsEnum {
            a::Columns::BId
        }
    }

    impl ManyToManyRelation<b::Entity, r::Entity> for a::Entity {
        fn own_relation_id_column() -> <r::Entity as TableEntity>::ColumnsEnum {
            r::Columns::BId
        }

        fn other_entity_id(entity: r::Entity) -> Uuid {
            entity.b_id
        }
    }
    impl ManyToManyRelation<a::Entity, r::Entity> for b::Entity {
        fn own_relation_id_column() -> <r::Entity as TableEntity>::ColumnsEnum {
            r::Columns::AId
        }

        fn other_entity_id(entity: r::Entity) -> Uuid {
            entity.a_id
        }
    }

    #[test]
    fn one_to_x() {
        let b_aid = <a::Entity as OneToXRelation<b::Entity>>::target_relation_id_column();
        let a_bid = <b::Entity as OneToXRelation<a::Entity>>::target_relation_id_column();

        assert!(b_aid == b::Columns::AId);
        assert!(a_bid == a::Columns::BId);
    }

    #[test]
    fn many_to_many() {
        let r_bid =
            <a::Entity as ManyToManyRelation<b::Entity, r::Entity>>::own_relation_id_column();
        let r_aid =
            <b::Entity as ManyToManyRelation<a::Entity, r::Entity>>::own_relation_id_column();

        assert!(r_bid == r::Columns::BId);
        assert!(r_aid == r::Columns::AId);
    }
}
