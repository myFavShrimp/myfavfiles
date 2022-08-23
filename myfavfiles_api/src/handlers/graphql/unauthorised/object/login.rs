use std::ops::DerefMut;

use juniper::FieldResult;
use sea_query::{Expr, PostgresQueryBuilder, Query};

use crate::{
    auth::token::Token,
    database::{
        driver::bind_query_as,
        entities::{self, TableEntity},
    },
    handlers::graphql::unauthorised::Context,
};

const USERNAME_PASSWORD_WRONG_ERROR_MESSAGE: &str = "Username or password wrong!";

pub async fn perform_login(
    ctx: &Context,
    username: String,
    password: String,
) -> FieldResult<String> {
    use chrono::Local;

    let (sql, values) = Query::select()
        .columns(entities::user::Entity::all_columns())
        .from(entities::user::Columns::Table)
        .and_where(Expr::col(entities::user::Columns::Name).eq(username))
        .and_where(Expr::col(entities::user::Columns::Password).eq(password))
        .build(PostgresQueryBuilder);

    let mut mutex = ctx.database_connection.lock().await;
    let conn = mutex.deref_mut();

    let query = bind_query_as(sqlx::query_as::<_, entities::user::Entity>(&sql), &values);
    let user = query
        .fetch_one(conn)
        .await
        .map_err(|_| USERNAME_PASSWORD_WRONG_ERROR_MESSAGE)?;

    let tok = Token {
        sub: user.id,
        jti: user.id,
        exp: (Local::now().timestamp() * 2) as usize,
    };

    let encoded = tok
        .encode(&ctx.app_state._config.jwt_secret)
        .map_err(|_| USERNAME_PASSWORD_WRONG_ERROR_MESSAGE)?;
    Ok(encoded)
}
