use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, Value, Values};

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
