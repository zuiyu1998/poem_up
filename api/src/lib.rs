use crate::error::Result;
use config::Config;
use entity::sea_orm::Database;
use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Server};

pub mod config;
pub mod error;

pub mod app;
pub mod middlewares;
pub mod users;

pub fn config() -> Result<Config> {
    dotenvy::dotenv().ok();

    #[cfg(feature = "debug")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = config::Config::init()?;
    Ok(config)
}

pub async fn init() -> anyhow::Result<()> {
    let config = config()?;

    let conn = Database::connect(&config.database_url).await?;

    let service = poem_up_service::Service::new(conn);

    let app = app::new().data(service).with(Tracing);
    let server = Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.host, config.port
    )));
    server.run(app).await?;

    Ok(())
}
