use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct MiscApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> MiscApi<'client> {
    pub async fn list_supported_countries(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::GET,
                "/checkout/supported_countries",
                query_params,
                body,
                ext_path,
            )
            .await
    }
}
