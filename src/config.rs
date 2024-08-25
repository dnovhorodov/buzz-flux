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
    pub update_interval: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HtmlSource {
    pub name: String,
    pub url: String,
    pub selector: String,
    pub update_interval: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub update_interval: String,
    pub max_concurrent_crawlers: u8,
}

pub trait Source {
    fn get_name(&self) -> &str;
    fn get_update_interval(&self, global_interval: String) -> Duration {
        let interval = self.get_specific_update_interval().unwrap_or(&global_interval);
        parse_interval(interval).unwrap_or_else(|_| parse_interval(&global_interval).unwrap())
    }

    fn get_specific_update_interval(&self) -> Option<&str>;
}

impl Source for RssSource {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_specific_update_interval(&self) -> Option<&str> {
        self.update_interval.as_deref()
    }
}

impl Source for HtmlSource {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_specific_update_interval(&self) -> Option<&str> {
        self.update_interval.as_deref()
    }
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

        // Validate update interval format
        parse_interval(&self.settings.update_interval)?;
        for source in &self.sources.rss {
            if let Some(ref interval) = source.update_interval {
                parse_interval(interval)?;
            }
        }
        for source in &self.sources.html {
            if let Some(ref interval) = source.update_interval {
                parse_interval(interval)?;
            }           
        }

        Ok(())
    }
    
    
}

fn parse_interval(interval: &str) -> Result<Duration> {
    let duration = match interval.chars().last()
    {
        Some('s') => {
            let seconds = interval.trim_end_matches('s').parse::<u64>()
                .context("Invalid seconds in update_interval")?;
            Duration::from_secs(seconds)
        }
        Some('m') => {
            let minutes = interval.trim_end_matches('m').parse::<u64>()
                .context("Invalid minutes in update_interval")?;
            Duration::from_secs(minutes * 60)
        }
        Some('h') => {
            let hours = interval.trim_end_matches('h').parse::<u64>()
                .context("Invalid hours in update_interval")?;
            Duration::from_secs(hours * 3600)
        }
        _ => {
            bail!("Invalid update_interval format: {}", interval);
        }
    };

    Ok(duration)
}
