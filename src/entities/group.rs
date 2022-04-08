use uuid::Uuid;

#[allow(dead_code)]
pub enum Columns {
    Table,
    Id,
    Name,
}

crate::entities::macros::impl_iden!{
    Columns,
    Table => "user",
    Id => "id",
    Name => "name",
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Entity {
    id: Uuid,
    name: String,
}
