use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct CheckoutApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> CheckoutApi<'client> {
    pub async fn create(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::POST, "/checkouts", query_params, body, ext_path)
            .await
    }
}
