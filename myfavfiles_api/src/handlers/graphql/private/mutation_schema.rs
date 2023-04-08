use super::{object::group::GroupCreaionInput, Context};

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_group<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        group: GroupCreaionInput,
    ) -> async_graphql::Result<String> {
        let context = context.data::<Context>()?;
        let current_user = context.session_token.sub;

        println!(
            "`{current_user}` wants to create the group `{}`",
            group.name,
        );

        Ok("test".to_owned())
    }
}
