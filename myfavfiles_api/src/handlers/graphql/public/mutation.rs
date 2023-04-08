use crate::handlers::graphql::public::object;

use super::Context;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn register<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        username: String,
        password: String,
    ) -> async_graphql::Result<bool> {
        let context = context.data::<Context>()?;
        object::user::perform_registration(context, username, password).await?;
        Ok(true)
    }
}
