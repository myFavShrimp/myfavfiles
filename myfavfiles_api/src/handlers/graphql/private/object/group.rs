use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct GroupCreaionInput {
    pub name: String,
}
