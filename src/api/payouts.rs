use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct PayoutApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> PayoutApi<'client> {
    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/payouts", query_params, body, ext_path)
            .await
    }
}
