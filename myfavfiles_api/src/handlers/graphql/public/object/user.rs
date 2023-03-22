use std::{ops::DerefMut, sync::Arc};

use axum::http::StatusCode;
use juniper::FieldResult;
use sea_query::{Expr, PostgresQueryBuilder, Query};

use crate::{
    auth::token::Token,
    database::{
        actions,
        driver::bind_query_as,
        entities::{self, TableEntity},
        password,
    },
    handlers::graphql::public::Context,
};

const USERNAME_PASSWORD_WRONG_ERROR_MESSAGE: &str = "Username or password wrong!";
const USERNAME_ALREADY_EXISTS_ERROR_MESSAGE: &str = "The username already exists!";

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
        .build(PostgresQueryBuilder);

    let mut mutex = ctx.database_connection.lock().await;
    let conn = mutex.deref_mut();

    let query = bind_query_as(sqlx::query_as::<_, entities::user::Entity>(&sql), &values);
    let user = query
        .fetch_one(conn)
        .await
        .map_err(|_| USERNAME_PASSWORD_WRONG_ERROR_MESSAGE)?;

    bcrypt::verify(&password, &user.password)
        .map_err(|_| ())
        .and_then(|ok| if ok { Ok(()) } else { Err(()) })
        .map_err(|_| USERNAME_PASSWORD_WRONG_ERROR_MESSAGE)?;

    if let Ok(Some(new_hash)) = password::new_password_hash_maybe(
        &password,
        &user.password,
        ctx.app_state.config.bcrypt_cost,
    ) {
        let (sql, values) = actions::build_update_query(
            entities::user::Columns::Table,
            vec![(entities::user::Columns::Password, new_hash.into())],
            entities::user::Columns::Id,
            user.id,
        );

        let conn = mutex.deref_mut();
        let query = bind_query_as(sqlx::query_as::<_, entities::user::Entity>(&sql), &values);
        query.fetch_one(conn).await.map_err(|_| {
            StatusCode::INTERNAL_SERVER_ERROR
                .canonical_reason()
                .unwrap()
        })?;
    };

    let tok = Token {
        sub: user.id,
        jti: user.id,
        exp: (Local::now().timestamp() * 2) as usize,
    };

    let encoded = tok
        .encode(&ctx.app_state.config.jwt_secret)
        .map_err(|_| USERNAME_PASSWORD_WRONG_ERROR_MESSAGE)?;
    Ok(encoded)
}

pub async fn perform_registration(
    ctx: &Context,
    username: String,
    password: String,
) -> FieldResult<Arc<entities::user::Entity>> {
    let password = bcrypt::hash(password, ctx.app_state.config.bcrypt_cost).map_err(|_| {
        StatusCode::INTERNAL_SERVER_ERROR
            .canonical_reason()
            .unwrap()
    })?;

    let values = vec![username.into(), password.into()];

    let (sql, values) = actions::build_insert_query(
        entities::user::Columns::Table,
        vec![
            entities::user::Columns::Name,
            entities::user::Columns::Password,
        ],
        values,
    );

    let mut mutex = ctx.database_connection.lock().await;
    let conn = mutex.deref_mut();

    let query = bind_query_as(sqlx::query_as::<_, entities::user::Entity>(&sql), &values);
    let user = query
        .fetch_one(conn)
        .await
        .map_err(|_| USERNAME_ALREADY_EXISTS_ERROR_MESSAGE)?;

    Ok(Arc::new(user))
}
