use uuid::Uuid;

#[allow(dead_code)]
pub enum Columns {
    Table,
    Id,
    Name,
}

crate::entities::macros::impl_iden!{
    Columns,
    Table => "group",
    Id => "id",
    Name => "name",
}

#[derive(sqlx::FromRow, Debug, Clone, juniper::GraphQLObject)]
#[allow(dead_code)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
}
