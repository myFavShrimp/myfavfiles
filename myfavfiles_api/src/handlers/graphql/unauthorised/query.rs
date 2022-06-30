use super::Context;

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    async fn hello(_context: &Context) -> String {
        "unauthorised".into()
    }
}
