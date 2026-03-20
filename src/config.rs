use std::env;

use anyhow::{Context, bail};

/// Runtime configuration loaded from environment variables.
///
/// Validated eagerly at startup — if anything is missing or
/// malformed we fail fast before the Lambda accepts requests.
#[derive(Debug, Clone)]
pub struct AppConfig {
    table_name: String,
    aws_region: String,
    log_level: String,
    max_page_size: usize,
}

impl AppConfig {
    /// Load configuration from real environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required variables are missing or
    /// invalid.
    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_lookup(|key| env::var(key).ok())
    }

    /// Load configuration using a custom lookup function.
    ///
    /// This allows testing without mutating process environment.
    ///
    /// # Errors
    ///
    /// Returns an error if required variables are missing or
    /// invalid.
    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> anyhow::Result<Self> {
        let table_name = get("TABLE_NAME").context("TABLE_NAME is required")?;

        let aws_region = get("AWS_REGION")
            .or_else(|| get("AWS_DEFAULT_REGION"))
            .unwrap_or_else(|| "ap-southeast-2".to_owned());

        let log_level = get("LOG_LEVEL").unwrap_or_else(|| "info".to_owned());

        let max_page_size = get("MAX_PAGE_SIZE")
            .unwrap_or_else(|| "50".to_owned())
            .parse::<usize>()
            .context("MAX_PAGE_SIZE must be a number")?;

        if max_page_size == 0 || max_page_size > 100 {
            bail!(
                "MAX_PAGE_SIZE must be between 1 and 100, \
                 got {max_page_size}"
            );
        }

        Ok(Self {
            table_name,
            aws_region,
            log_level,
            max_page_size,
        })
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn aws_region(&self) -> &str {
        &self.aws_region
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    pub fn max_page_size(&self) -> usize {
        self.max_page_size
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn env_with(vars: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = vars
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        move |key| map.get(key).cloned()
    }

    fn required_env() -> impl Fn(&str) -> Option<String> {
        env_with(&[("TABLE_NAME", "test-items")])
    }

    #[test]
    fn loads_with_required_vars() {
        let config = AppConfig::from_lookup(required_env()).unwrap();
        assert_eq!(config.table_name(), "test-items");
        assert_eq!(config.aws_region(), "ap-southeast-2");
        assert_eq!(config.log_level(), "info");
        assert_eq!(config.max_page_size(), 50);
    }

    #[test]
    fn fails_without_table_name() {
        let result = AppConfig::from_lookup(env_with(&[]));
        assert!(result.is_err());
    }

    #[test]
    fn respects_custom_region() {
        let config = AppConfig::from_lookup(env_with(&[
            ("TABLE_NAME", "t"),
            ("AWS_REGION", "us-east-1"),
        ]))
        .unwrap();
        assert_eq!(config.aws_region(), "us-east-1");
    }

    #[test]
    fn falls_back_to_default_region() {
        let config = AppConfig::from_lookup(env_with(&[
            ("TABLE_NAME", "t"),
            ("AWS_DEFAULT_REGION", "eu-west-1"),
        ]))
        .unwrap();
        assert_eq!(config.aws_region(), "eu-west-1");
    }

    #[test]
    fn rejects_zero_page_size() {
        let result =
            AppConfig::from_lookup(env_with(&[("TABLE_NAME", "t"), ("MAX_PAGE_SIZE", "0")]));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_page_size_over_100() {
        let result =
            AppConfig::from_lookup(env_with(&[("TABLE_NAME", "t"), ("MAX_PAGE_SIZE", "200")]));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_numeric_page_size() {
        let result =
            AppConfig::from_lookup(env_with(&[("TABLE_NAME", "t"), ("MAX_PAGE_SIZE", "abc")]));
        assert!(result.is_err());
    }
}
