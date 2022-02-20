use std::sync::Arc;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer, Scope};
use futures::executor::block_on;
use sea_orm::{Database, DatabaseConnection};
use crate::database::get_connection_pool;

mod config;
mod handlers;
mod database;

struct AppState {
    database_connection: Arc<DatabaseConnection>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            database_connection: Arc::new(block_on(Database::connect(config::Config::default().database_url)).expect("DATABASE CONNECTION"))
        }
    }
}

fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(get_scope())
        .app_data(web::Data::new(AppState::default()));
}

fn get_scope() -> Scope {
    web::scope("")
        .route("graphql", web::post().to(handlers::graphql))
        .service(
            Files::new("", "frontend")
                .index_file("index.html")
                .prefer_utf8(true)
                .default_handler(web::to(handlers::handler_404)),
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = config::Config::default();

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(initialize))
        .bind(config.address())?
        .run()
        .await
}
