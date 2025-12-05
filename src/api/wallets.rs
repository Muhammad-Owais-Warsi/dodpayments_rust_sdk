use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct WalletsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> WalletsApi<'client> {
    pub async fn list(
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

    pub async fn create(
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

    pub async fn retreive(
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
}
