use crate::handlers::graphql::public::object;

use super::Context;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn login<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        username: String,
        password: String,
    ) -> async_graphql::Result<String> {
        let context = context.data::<Context>()?;
        object::user::perform_login(context, username, password).await
    }
}
