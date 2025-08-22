use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct LicensesApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> LicensesApi<'client> {
    pub async fn activate(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::POST,
                "/licenses/activate",
                query_params,
                body,
                ext_path,
            )
            .await
    }

    pub async fn deactivate(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::POST,
                "/licenses/deactivate",
                query_params,
                body,
                ext_path,
            )
            .await
    }

    pub async fn validate(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::POST,
                "/licenses/validate",
                query_params,
                body,
                ext_path,
            )
            .await
    }

    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/license_keys", query_params, body, ext_path)
            .await
    }

    pub async fn retreive(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/license_keys", query_params, body, ext_path)
            .await
    }

    pub async fn update(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::PATCH, "/license_keys", query_params, body, ext_path)
            .await
    }

    pub async fn list_license_key_instances(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::GET,
                "/license_key_instances",
                query_params,
                body,
                ext_path,
            )
            .await
    }

    pub async fn list_license_key_instance(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::GET,
                "/license_key_instances",
                query_params,
                body,
                ext_path,
            )
            .await
    }

    pub async fn update_license_key_instance(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::PATCH,
                "/license_key_instances",
                query_params,
                body,
                ext_path,
            )
            .await
    }
}
