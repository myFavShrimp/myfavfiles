use std::{ops::DerefMut, sync::Arc};

use axum::http::StatusCode;
use mini_orm::entity::TableEntity;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;

use crate::{
    auth::token::Token,
    database::{actions, entities, password},
    handlers::graphql::public::Context,
};

const USERNAME_PASSWORD_WRONG_ERROR_MESSAGE: &str = "Username or password wrong!";
const USERNAME_ALREADY_EXISTS_ERROR_MESSAGE: &str = "The username already exists!";

pub async fn perform_login(
    ctx: &Context,
    username: String,
    password: String,
) -> async_graphql::Result<String> {
    use chrono::Local;

    let (sql, values) = Query::select()
        .columns(entities::user::Entity::all_columns())
        .from(entities::user::Iden::Table)
        .and_where(Expr::col(entities::user::Iden::Name).eq(username))
        .build_sqlx(PostgresQueryBuilder);

    let mut mutex = ctx.database_connection.lock().await;
    let conn = mutex.deref_mut();

    let query = sqlx::query_as_with::<_, entities::user::Entity, _>(&sql, values);
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
            entities::user::Iden::Table,
            vec![(entities::user::Iden::Password, new_hash.into())],
            entities::user::Iden::Id,
            user.id,
        );

        let conn = mutex.deref_mut();
        let query = sqlx::query_as_with::<_, entities::user::Entity, _>(&sql, values);
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
) -> async_graphql::Result<Arc<entities::user::Entity>> {
    let password = bcrypt::hash(password, ctx.app_state.config.bcrypt_cost).map_err(|_| {
        StatusCode::INTERNAL_SERVER_ERROR
            .canonical_reason()
            .unwrap()
    })?;

    let values = vec![username.into(), password.into()];

    let (sql, values) = actions::build_insert_query(
        entities::user::Iden::Table,
        vec![entities::user::Iden::Name, entities::user::Iden::Password],
        values,
    )?;

    let mut mutex = ctx.database_connection.lock().await;
    let conn = mutex.deref_mut();

    let query = sqlx::query_as_with::<_, entities::user::Entity, _>(&sql, values);
    let user = query
        .fetch_one(conn)
        .await
        .map_err(|_| USERNAME_ALREADY_EXISTS_ERROR_MESSAGE)?;

    Ok(Arc::new(user))
}
