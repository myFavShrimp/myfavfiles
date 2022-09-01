use juniper::FieldResult;

use crate::handlers::graphql::unauthorised::object;

use super::Context;

pub struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Mutation {
    async fn register(context: &Context, username: String, password: String) -> FieldResult<bool> {
        object::user::perform_registration(context, username, password).await?;
        Ok(true)
    }
}
