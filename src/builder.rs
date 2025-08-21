use crate::client::DodoPaymentsClient;
use reqwest::Client;

pub struct DodoPaymentsClientBuilder {
    enviroment: Option<String>,
    base_url: Option<String>,
    bearer_token: Option<String>,
    timeout: Option<u64>,
}

impl DodoPaymentsClientBuilder {
    pub fn new() -> Self {
        Self {
            enviroment: None,
            base_url: None,
            bearer_token: None,
            timeout: None,
        }
    }

    pub fn enviroment(mut self, env: &str) -> Self {
        self.enviroment = Some(env.to_string());
        self
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
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

    pub fn build(self) -> Result<DodoPaymentsClient, &'static str> {
        let env = self
            .enviroment
            .unwrap_or_else(|| "https://live.dodopayments.com".to_string());
        let token = self.bearer_token.ok_or("Bearer token is required")?;
        let mut builder = Client::builder();
        if let Some(secs) = self.timeout {
            builder = builder.timeout(std::time::Duration::from_secs(secs));
        }

        let client = builder.build().map_err(|_| "Failed to build client")?;

        Ok(DodoPaymentsClient {
            client,
            enviroment: env,
            bearer_token: token,
        })
    }
}
