use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct UsageEventsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> UsageEventsApi<'client> {
    pub async fn ingest(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::POST, "/events/ingest", query_params, body, ext_path)
            .await
    }

    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/events", query_params, body, ext_path)
            .await
    }

    pub async fn retrieve(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/events", query_params, body, ext_path)
            .await
    }
}
