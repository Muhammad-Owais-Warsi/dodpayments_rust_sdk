use crate::api::disputes::DisputesApi;
use crate::api::misc::MiscApi;
use crate::api::payments::PaymentsApi;
use crate::api::payouts::PayoutApi;
use crate::api::refunds::RefundsApi;

use reqwest::{Client, Method};
use std::collections::HashMap;
use thiserror::Error;

pub struct DodoPaymentsClient {
    pub(crate) client: Client,
    pub(crate) enviroment: String,
    pub(crate) bearer_token: String,
}

pub enum ResponseData {
    Text(String),
    Blob(bytes::Bytes),
}

#[derive(Debug, Error)]
pub enum DodoError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },
}

impl DodoPaymentsClient {
    pub async fn request(
        &self,
        method: Method,
        path: &str,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let mut url = format!("{}{}", self.enviroment, path);

        if let Some(ext_path) = ext_path {
            url = format!("{}/{}", url, ext_path);
        }

        let mut req = self
            .client
            .request(method, &url)
            .bearer_auth(&self.bearer_token);

        if let Some(params) = query_params {
            req = req.query(&params);
        }

        if let Some(b) = body {
            req = req.json(&b);
        }

        let resp = req.send().await?;

        let status = resp.status();
        let headers = resp.headers().clone();

        if !status.is_success() {
            let bytes = resp.bytes().await?;
            return Err(DodoError::Api {
                status: status.as_u16(),
                message: String::from_utf8_lossy(&bytes).to_string(),
            });
        }

        if let Some(content_type) = headers.get("content-type") {
            if content_type.to_str().unwrap_or("").starts_with("text/")
                || content_type.to_str().unwrap_or("").contains("json")
            {
                let text = resp.text().await?;
                return Ok(ResponseData::Text(text));
            }
        }

        let bytes = resp.bytes().await?;
        Ok(ResponseData::Blob(bytes))
    }

    pub fn payments(&self) -> PaymentsApi {
        PaymentsApi { client: self }
    }

    pub fn payouts(&self) -> PayoutApi {
        PayoutApi { client: self }
    }

    pub fn refunds(&self) -> RefundsApi {
        RefundsApi { client: self }
    }

    pub fn disputes(&self) -> DisputesApi {
        DisputesApi { client: self }
    }

    pub fn misc(&self) -> MiscApi {
        MiscApi { client: self }
    }
}
