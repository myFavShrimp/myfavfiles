use super::Context;


pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    fn hello(context: &Context) -> String {
        context.app_state.as_ref().config.database_url.clone()
    }
}
