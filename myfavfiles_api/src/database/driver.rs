#![allow(clippy::manual_map)]
sea_query::sea_query_driver_postgres!();

pub use sea_query_driver_postgres::bind_query_as;
