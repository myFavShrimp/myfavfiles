use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer, Scope};

mod config;
mod handlers;

fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(get_scope())
        // .app_data(web::Data::new());
    ;
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
