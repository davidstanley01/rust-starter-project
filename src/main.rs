use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;

use tracing::info;

use starter::{AppConfig, ApplicationServer, Logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    let _guard = Logger::init(config.cargo_env);

    ApplicationServer::serve(config)
        .await
        .context("could not initialize application routes")?;

    info!("Server started and listening");

    Ok(())
}
