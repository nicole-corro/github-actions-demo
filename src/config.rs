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
    /// Load configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required variables are missing or
    /// invalid.
    pub fn from_env() -> anyhow::Result<Self> {
        let table_name = env::var("TABLE_NAME").context("TABLE_NAME is required")?;

        let aws_region = env::var("AWS_REGION")
            .or_else(|_| env::var("AWS_DEFAULT_REGION"))
            .unwrap_or_else(|_| "ap-southeast-2".to_owned());

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_owned());

        let max_page_size = env::var("MAX_PAGE_SIZE")
            .unwrap_or_else(|_| "50".to_owned())
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
    use super::*;
    use std::env;

    // SAFETY: These tests mutate environment variables which is
    // unsafe in multi-threaded contexts. Each test clears its own
    // state. Run with `cargo test -- --test-threads=1` or use
    // serial_test for isolation.
    fn set_required_env() {
        unsafe { env::set_var("TABLE_NAME", "test-items") };
    }

    fn clear_env() {
        unsafe {
            env::remove_var("TABLE_NAME");
            env::remove_var("AWS_REGION");
            env::remove_var("AWS_DEFAULT_REGION");
            env::remove_var("LOG_LEVEL");
            env::remove_var("MAX_PAGE_SIZE");
        }
    }

    #[test]
    fn loads_with_required_vars() {
        clear_env();
        set_required_env();
        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.table_name(), "test-items");
        assert_eq!(config.aws_region(), "ap-southeast-2");
        assert_eq!(config.log_level(), "info");
        assert_eq!(config.max_page_size(), 50);
        clear_env();
    }

    #[test]
    fn fails_without_table_name() {
        clear_env();
        let result = AppConfig::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn respects_custom_region() {
        clear_env();
        set_required_env();
        unsafe { env::set_var("AWS_REGION", "us-east-1") };
        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.aws_region(), "us-east-1");
        clear_env();
    }

    #[test]
    fn rejects_zero_page_size() {
        clear_env();
        set_required_env();
        unsafe { env::set_var("MAX_PAGE_SIZE", "0") };
        let result = AppConfig::from_env();
        assert!(result.is_err());
        clear_env();
    }

    #[test]
    fn rejects_page_size_over_100() {
        clear_env();
        set_required_env();
        unsafe { env::set_var("MAX_PAGE_SIZE", "200") };
        let result = AppConfig::from_env();
        assert!(result.is_err());
        clear_env();
    }

    #[test]
    fn rejects_non_numeric_page_size() {
        clear_env();
        set_required_env();
        unsafe { env::set_var("MAX_PAGE_SIZE", "abc") };
        let result = AppConfig::from_env();
        assert!(result.is_err());
        clear_env();
    }
}
