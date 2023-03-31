use juniper::FieldResult;

use super::{object::group::GroupCreaionInput, Context};

pub struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Mutation {
    async fn create_group(context: &Context, group: GroupCreaionInput) -> FieldResult<String> {
        let current_user = context.session_token.sub;

        println!(
            "`{current_user}` wants to create the group `{}`",
            group.name,
        );

        Ok("test".to_owned())
    }
}
