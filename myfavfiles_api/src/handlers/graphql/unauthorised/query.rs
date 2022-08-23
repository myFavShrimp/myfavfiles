use juniper::FieldResult;

use crate::handlers::graphql::unauthorised::object;

use super::Context;

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    async fn login(context: &Context, username: String, password: String) -> FieldResult<String> {
        object::login::perform_login(context, username, password).await
    }
}
