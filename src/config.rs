use crate::error::PubSubError;
use std::env;

pub struct Config {
    pub project_id: String,
    pub credentials_path: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, PubSubError> {
        let project_id = env::var("GCP_PROJECT_ID")
            .or_else(|_| env::var("GOOGLE_CLOUD_PROJECT"))
            .map_err(|_| {
                PubSubError::ConfigError(
                    "GCP_PROJECT_ID or GOOGLE_CLOUD_PROJECT environment variable not set".to_string(),
                )
            })?;

        let credentials_path = env::var("GOOGLE_APPLICATION_CREDENTIALS").ok();

        Ok(Config {
            project_id,
            credentials_path,
        })
    }

    pub fn new(project_id: String, credentials_path: Option<String>) -> Self {
        Config {
            project_id,
            credentials_path,
        }
    }
}
