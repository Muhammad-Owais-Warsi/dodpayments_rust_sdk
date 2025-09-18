use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct CustomersApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> CustomersApi<'client> {
    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/customers", query_params, body, ext_path)
            .await
    }

    pub async fn create(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::POST, "/customers", query_params, body, ext_path)
            .await
    }

    pub async fn retreive(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/customers", query_params, body, ext_path)
            .await
    }

    pub async fn update(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::PATCH, "/customers", query_params, body, ext_path)
            .await
    }

    pub async fn create_customer_portal(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/customer-portal/session", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::POST,
                "/customers",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }

    pub async fn get_customer_wallets(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/wallets", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::GET,
                "/customers",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }

    pub async fn get_ledger_entries(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/wallets/ledger-entries", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::GET,
                "/customers",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }

    pub async fn create_ledger_entry(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/wallets/ledger-entries", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::POST,
                "/customers",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await        
    }
}
