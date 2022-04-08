use uuid::Uuid;


#[allow(dead_code)]
pub enum Columns {
    Table,
    Id,
    Name,
    Password,
    IsAdmin,
}

crate::entities::macros::impl_iden!{
    Columns,
    Table => "user",
    Id => "id",
    Name => "name",
    Password => "password",
    IsAdmin => "is_admin",
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Entity {
    id: Uuid,
    name: String,
    password: String,
    is_admin: bool,
}
