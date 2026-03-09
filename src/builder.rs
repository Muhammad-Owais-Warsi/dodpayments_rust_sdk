use crate::{client::DodoPaymentsClient, DodoError};
use reqwest::Client;

pub struct DodoPaymentsClientBuilder {
    environment: Option<String>,
    bearer_token: Option<String>,
    timeout: Option<u64>,
}

impl DodoPaymentsClientBuilder {
    pub fn new() -> Self {
        Self {
            environment: None,
            bearer_token: None,
            timeout: None,
        }
    }

    pub fn enviroment(mut self, env: &str) -> Self {
        self.environment = Some(env.to_string());
        self
    }

    pub fn bearer_token(mut self, token: &str) -> Self {
        self.bearer_token = Some(token.to_string());
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = Some(secs);
        self
    }

    pub fn build(self) -> Result<DodoPaymentsClient, DodoError> {
        let env = match self.environment {
            Some(env) if env == "test_mode" => "https://test.dodopayments.com".to_string(),
            Some(env) if env == "live_mode" => "https://live.dodopayments.com".to_string(),
            None => "https://live.dodopayments.com".to_string(),
            Some(_) => {
                return Err(DodoError::Custom {
                    message: "This mode is not supported".to_string(),
                })
            }
        };

        let token = self.bearer_token.ok_or_else(|| DodoError::Custom {
            message: "Bearer token is required.".to_string(),
        })?;
        let mut builder = Client::builder();
        if let Some(secs) = self.timeout {
            builder = builder.timeout(std::time::Duration::from_secs(secs));
        }

        let client = builder.build().map_err(|e| DodoError::Custom {
            message: e.to_string(),
        })?;

        Ok(DodoPaymentsClient {
            client,
            environment: env,
            bearer_token: token,
        })
    }
}
