use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct SubscriptionsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> SubscriptionsApi<'client> {
    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/subscriptions", query_params, body, ext_path)
            .await
    }

    pub async fn create(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::POST, "/subscriptions", query_params, body, ext_path)
            .await
    }

    pub async fn retrieve(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/subscriptions", query_params, body, ext_path)
            .await
    }

    pub async fn change_plan(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/charge", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };
        self.client
            .request(
                Method::POST,
                "/subscriptions",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }

    pub async fn update(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::PATCH,
                "/subscriptions",
                query_params,
                body,
                ext_path,
            )
            .await
    }

    pub async fn charge(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/charge", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::POST,
                "/subscriptions",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }

    pub async fn get_usage_history(
        &self, 
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/usage-history", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::GET,
                "/subscriptions",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }
}
