use tracing_subscriber::fmt;
use crate::config::Config;

mod config;
mod crawler;
mod parser;
mod scheduler;
mod db;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    // Initialize logging
    fmt::init();

    // Load configuration
    let config = Config::load_config("config.toml")?;
    config.validate()?;

    // Start the crawling process
    //scheduler::start(config).await?;

    Ok(())
}
