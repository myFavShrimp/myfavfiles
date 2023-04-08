use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, SimpleExpr, Value};
use sea_query_binder::{SqlxBinder, SqlxValues};
use uuid::Uuid;

pub fn build_select_query<ColumnsEnum, IdType>(
    columns: Vec<ColumnsEnum>,
    table: ColumnsEnum,
    id_column: ColumnsEnum,
    ids_to_load: Option<Vec<IdType>>,
) -> (String, SqlxValues)
where
    ColumnsEnum: Iden + 'static,
    IdType: Into<Value>,
{
    match ids_to_load {
        Some(ids_to_load) => Query::select()
            .columns(columns)
            .from(table)
            .and_where(Expr::col(id_column).is_in(ids_to_load))
            .build_sqlx(PostgresQueryBuilder),
        None => Query::select()
            .columns(columns)
            .from(table)
            .build_sqlx(PostgresQueryBuilder),
    }
}

pub fn build_insert_query<ColumnsEnum>(
    table: ColumnsEnum,
    columns: Vec<ColumnsEnum>,
    values: Vec<SimpleExpr>,
) -> (String, SqlxValues)
where
    ColumnsEnum: Iden + 'static,
{
    Query::insert()
        .into_table(table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning_all()
        .build_sqlx(PostgresQueryBuilder)
}

pub fn build_update_query<ColumnsEnum>(
    table: ColumnsEnum,
    values: Vec<(ColumnsEnum, SimpleExpr)>,
    id_column: ColumnsEnum,
    id: Uuid,
) -> (String, SqlxValues)
where
    ColumnsEnum: Iden + 'static,
{
    Query::update()
        .table(table)
        .values(values)
        .and_where(Expr::col(id_column).eq(id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder)
}
