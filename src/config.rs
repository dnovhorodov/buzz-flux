use std::fs;
use anyhow::{Result, bail, Context};
use std::time::Duration;
use url::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sources: Sources,
    pub settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct Sources {
    pub rss: Vec<RssSource>,
    pub html: Vec<HtmlSource>,
}

#[derive(Debug, Deserialize)]
pub struct RssSource {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct HtmlSource {
    pub name: String,
    pub url: String,
    pub selector: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub update_interval: String,
}

impl Config {
    pub fn load_config(file_path: &str) -> Result<Self> {
        let config_content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read config file: {}", file_path))?;

        let config = toml::from_str(&config_content)
            .with_context(|| "Failed to parse config.toml")?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        // Validate RSS feed URLs
        for feed in &self.sources.rss {
            Url::parse(&feed.url).with_context(|| format!("Invalid RSS feed URL: {:?}", feed))?;
        }

        // Validate website URLs
        for site in &self.sources.html {
            Url::parse(&site.url).with_context(|| format!("Invalid website URL: {:?}", site.url))?;
            if site.selector.is_empty() {
                bail!("CSS selector cacnnot be empty for site: {}", site.name);
            }
        }

        // Validate update interval format (e.g., "10m", "1h")
        let _interval = self.parse_update_interval()?;

        Ok(())
    }

    fn parse_update_interval(&self) -> Result<Duration> {
        let interval = &self.settings.update_interval;
        let duration = if interval.ends_with('m') {
            let minutes = interval.trim_end_matches('m').parse::<u64>()
                .context("Invalid minutes in update_interval")?;
            Duration::from_secs(minutes * 60)
        } else if interval.ends_with('h') {
            let hours = interval.trim_end_matches('h').parse::<u64>()
                .context("Invalid hours in update_interval")?;
            Duration::from_secs(hours * 3600)
        } else {
            bail!("Invalid update_interval format: {}", interval);
        };

        Ok(duration)
    }
}
