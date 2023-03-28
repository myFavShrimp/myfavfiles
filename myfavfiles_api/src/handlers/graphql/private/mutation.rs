use juniper::FieldResult;

use super::Context;

pub struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Mutation {
    async fn create_group(context: &Context) -> FieldResult<String> {
        Ok("test".to_owned())
    }
}
