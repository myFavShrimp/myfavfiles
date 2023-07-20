#[derive(async_graphql::InputObject)]
pub struct GroupRoleCreationInput {
    pub name: String,
    pub group_id: uuid::Uuid,
}
