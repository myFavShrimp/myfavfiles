use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, Value, Values};
use uuid::Uuid;

pub fn build_select_query<ColumnsEnum, IdType, ColumnsEnumId>(
    columns: Vec<ColumnsEnum>,
    table: ColumnsEnum,
    id_column: ColumnsEnumId,
    ids_to_load: Option<Vec<IdType>>,
) -> (String, Values)
where
    ColumnsEnum: Iden + 'static,
    IdType: Into<Value>,
    ColumnsEnumId: Iden + 'static,
{
    match ids_to_load {
        Some(ids_to_load) => Query::select()
            .columns(columns)
            .from(table)
            .and_where(Expr::col(id_column).is_in(ids_to_load))
            .build(PostgresQueryBuilder),
        None => Query::select()
            .columns(columns)
            .from(table)
            .build(PostgresQueryBuilder),
    }
}

pub fn build_insert_query<ColumnsEnum>(
    table: ColumnsEnum,
    columns: Vec<ColumnsEnum>,
    values: Vec<Value>,
) -> (String, Values)
where
    ColumnsEnum: Iden + 'static,
{
    Query::insert()
        .into_table(table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning_all()
        .build(PostgresQueryBuilder)
}

pub fn build_update_query<ColumnsEnum>(
    table: ColumnsEnum,
    values: Vec<(ColumnsEnum, Value)>,
    id_column: ColumnsEnum,
    id: Uuid,
) -> (String, Values)
where
    ColumnsEnum: Iden + 'static,
{
    Query::update()
        .table(table)
        .values(values)
        .and_where(Expr::col(id_column).eq(id))
        .returning_all()
        .build(PostgresQueryBuilder)
}
