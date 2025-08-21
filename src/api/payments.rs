use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct PaymentsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> PaymentsApi<'client> {
    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/payments", query_params, body, ext_path)
            .await
    }

    pub async fn create(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::POST, "/payments", query_params, body, ext_path)
            .await
    }

    pub async fn retrieve(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/payments", query_params, body, ext_path)
            .await
    }

    pub async fn retrieve_invoice(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::GET,
                "/invoices/payments",
                query_params,
                body,
                ext_path,
            )
            .await
    }

    pub async fn retrieve_line_items(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = ext_path
            .map(|p| format!("{}/line-items", p))
            .unwrap_or_else(|| "line-items".to_string());

        self.client
            .request(
                Method::GET,
                "/payments",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }
}
