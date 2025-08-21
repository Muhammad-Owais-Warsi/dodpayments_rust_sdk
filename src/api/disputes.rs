use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct DisputesApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> DisputesApi<'client> {
    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/disputes", query_params, body, ext_path)
            .await
    }

    pub async fn retreive(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/disputes", query_params, body, ext_path)
            .await
    }
}
