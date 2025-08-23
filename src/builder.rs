use crate::client::DodoPaymentsClient;
use reqwest::Client;

pub struct DodoPaymentsClientBuilder {
    enviroment: Option<String>,
    bearer_token: Option<String>,
    timeout: Option<u64>,
}

impl DodoPaymentsClientBuilder {
    pub fn new() -> Self {
        Self {
            enviroment: None,
            bearer_token: None,
            timeout: None,
        }
    }

    pub fn enviroment(mut self, env: &str) -> Self {
        self.enviroment = Some(env.to_string());
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
        let env = match self.enviroment {
            Some(env) if env == "test_mode" => "https://test.dodopayments.com".to_string(),
            Some(env) if env == "live_mode" => "https://live.dodopayments.com".to_string(),
            None => "https://live.dodopayments.com".to_string(),
            Some(_) => panic!("This mode is not supported"),
        };

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
