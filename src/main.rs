use crate::config::Config;
use tracing_subscriber::fmt;

mod config;
mod crawler;
mod db;
mod parser;
mod scheduler;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    fmt::init();

    // Load configuration
    let config = Config::load_config("config.toml")?;
    config.validate()?;

    println!("{:#?}", config);
    // Start the crawling process
    //scheduler::start(config).await?;

    Ok(())
}
